use crate::parser::{Args, Command};
use std::collections::HashMap;
use std::path::Path;

static COMMON_MEM_SEG: [(&str, &str); 4] = [
    ("argument", "ARG"),
    ("local", "LCL"),
    ("that", "THAT"),
    ("this", "THIS"),
];

fn common_segment(mem_seg: &str) -> Option<&str> {
    for (mem, hack_mem) in COMMON_MEM_SEG {
        if mem == mem_seg {
            return Some(hack_mem);
        }
    }
    None
}

pub struct CodeWriter<P>
where
    P: AsRef<Path>,
{
    filename: P,
}

impl<P: AsRef<Path>> CodeWriter<P> {
    pub fn init(filename: P) -> Self {
        Self { filename }
    }
    pub fn to_assembly<'a>(args: Args) -> Vec<String> {
        match args.command() {
            Some(Command::PUSH) => {
                let offset = args.three().as_ref().unwrap();
                let memory_segment = args.two().as_ref().unwrap();
                let common_hack_memory = common_segment(memory_segment);
                match common_hack_memory {
                    Some(hack_mem) => {
                        // push common_memory_segment offset
                        return vec![
                            format!("@{offset}"),
                            "D=A".to_string(), // D = offset
                            format!("@{hack_mem}"),
                            "A=D+M".to_string(), // addr = segment_pointer + offset
                            "D=M".to_string(),   // D = *addr
                            "@SP".to_string(),
                            "A=M".to_string(),
                            "M=D".to_string(), // *SP = *addr
                            "@SP".to_string(),
                            "M=M+1".to_string(), // SP ++
                        ];
                    }
                    None => {
                        return vec![];
                    }
                }
            }
            _ => return vec![],
        }

        //                         return vec![
        //                             format!("@{offset}"),
        //                             "D=A".to_string(), // D = offset
        //                             format!("@{mem_seg}"),
        //                             "A=D+M".to_string(), // addr = segment_pointer + offset
        //                             "D=M".to_string(),   // D = *addr
        //                             "@SP".to_string(),
        //                             "A=M".to_string(),
        //                             "M=D".to_string(), // *SP = *addr
        //                             "@SP".to_string(),
        //                             "M=M+1".to_string(), // SP ++
        //                         ];
        //                     }
        //                     None => {
        //                         // push constant offset
        //                         if memory_segment == "constant" {
        //                             return vec![
        //                                 format!("@{offset}"),
        //                                 "D=A".to_string(),
        //                                 "@SP".to_string(),
        //                                 "A=M".to_string(),
        //                                 "M=D".to_string(),
        //                                 "@SP".to_string(),
        //                                 "M=M+1".to_string(),
        //                             ];
        //                         } else
        //                     }
        //                 }
        //             } else {
        //                 return vec![];
        //             }
        //         } else {
        //             panic!("No memory segment.");
        //         }
        //     }
        //     None => return vec![],
        //     _ => return vec![],
        // };
    }
}
