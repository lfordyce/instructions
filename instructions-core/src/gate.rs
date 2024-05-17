use std::cell::Cell;
use std::collections::HashMap;
use Gate::*;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Gate {
    Simple(u64, u64, u64),
    Function(Vec<u64>, Vec<u64>, Vec<Gate>),
}

fn translate_gates<'s>(
    gates: Vec<Gate>,
    output_input: Vec<u64>,
    translated_gates: &'s Cell<usize>,
) -> impl Iterator<Item = Gate> + 's {
    gates
        .into_iter()
        .map(move |gate| translate_gate(&gate, &output_input, translated_gates))
}

fn translate_gate<'s>(
    gate: &'s Gate,
    output_input: &[u64],
    translated_gates: &'s Cell<usize>,
) -> Gate {
    match gate {
        Simple(out, in1, in2) => {
            translated_gates.set(translated_gates.get() + 1);
            Simple(
                output_input[*out as usize],
                output_input[*in1 as usize],
                output_input[*in2 as usize],
            )
        }
        Function(outs, ins, gates) => {
            translated_gates.set(translated_gates.get() + 1);
            let translated_outputs = outs.iter().map(|&val| output_input[val as usize]).collect();
            let translated_inputs = ins.iter().map(|&val| output_input[val as usize]).collect();
            Function(translated_outputs, translated_inputs, gates.clone())
        }
    }
}

fn flatten_gates<'a>(
    gates: impl Iterator<Item = Gate> + 'a,
    _not_used: &'a HashMap<String, u64>,
    translated: &'a Cell<usize>,
) -> impl Iterator<Item = Gate> + 'a {
    gates.flat_map(move |gate| -> Box<dyn Iterator<Item = Gate> + 'a> {
        flatten(gate, _not_used, translated)
    })
}

fn flatten<'a>(
    gate: Gate,
    _not_used: &'a HashMap<String, u64>,
    translated: &'a Cell<usize>,
) -> Box<dyn Iterator<Item = Gate> + 'a> {
    match gate {
        Simple(..) => Box::new(std::iter::once(gate.clone())),
        Function(outs, ins, gates) => {
            let output_inputs = [outs.to_vec(), ins.to_vec()].concat();
            Box::new(
                translate_gates(gates, output_inputs, translated)
                    .flat_map(move |inner_gate| flatten(inner_gate, _not_used, translated)),
            )
        }
    }
}
