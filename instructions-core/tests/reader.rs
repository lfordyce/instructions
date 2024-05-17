// use std::io::Read;
//
// type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
//
// #[test]
// fn chuncked_reader() -> Result<(), Error> {
//     let path = match std::env::args_os().nth(1) {
//         Some(path) => std::path::PathBuf::from(path),
//         None => {
//             return Err(Error::from("Usage: cmd <path>"));
//         }
//     };
//     let mut input: Box<dyn std::io::Read + 'static> = if path.as_os_str() == "-" {
//         Box::new(std::io::stdin())
//     } else {
//         match std::fs::File::open(&path) {
//             Ok(file) => Box::new(file),
//             Err(err) => {
//                 let msg = format!("{}: {}", path.display(), err);
//                 return Err(Error::from(msg));
//             }
//         }
//     };
//
//     const BUFFER_SIZE: usize = 8 << 10;
//     let mut buffer_vec = Vec::with_capacity(BUFFER_SIZE);
//     let operation = |bytes: &[u8]| println!("{}", bytes.len());
//     loop {
//         match input
//             .by_ref()
//             .take(BUFFER_SIZE as u64)
//             .read_to_end(&mut buffer_vec)
//         {
//             Err(err) => return Err(Error::from(err)),
//             Ok(chunk_size) => {
//                 if chunk_size == 0 {
//                     break;
//                 }
//
//                 operation(&buffer_vec);
//
//                 if chunk_size < BUFFER_SIZE {
//                     break;
//                 }
//
//                 buffer_vec.clear();
//             }
//         }
//     }
//
//     Ok(())
//
//     /*
//     let mut data_src_fd;
//     if data_src.eq("-") {
//         let stdin = io::stdin();
//         let handler = stdin.lock();
//         data_src_fd = handler.by_ref();
//     } else {
//         let mut handler2 = match File::open(&data_src) {
//             Ok(file) => file,
//             Err(error_description) => {
//                 eprintln!(
//                     "Unable to open file {} ({})",
//                     data_src,
//                     error_description.to_string()
//                 );
//                 process::exit(-1);
//             }
//         };
//         data_src_fd = handler2.by_ref();
//     }
//     */
// }
