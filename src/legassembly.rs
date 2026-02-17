# mod legassembly{
# 
#    use self::disassembly::decode;
#    use std::collections::HashMap;
#     
#   #[derive(Debug)]
#    enum ParsedLine {
#        Label(String),
#        Instruction(Instruction),
#        Directive(Directive),
#        Empty,
#    }
#
#    #[derive(Debug)]
#    enum Directive {
#        Data,
#        Text,
#        Org(u8),
#        Byte(u8),
#    }
#
#    #[derive(Debug)]
#    enum Operand {
#        Source(u8),
#        Immediate(u8),
#        IndirectAddr(String),
#        MemLabel(String),
#    }
#
#    #[derive(Debug)]
#    struct SourceOperand{
#        imm_flag: u8,
#        src: Operand
#    }
#    impl SourceOperand {
#        pub fn new(imm_flag: u8, src: Operand) -> Self{
#            Self{imm_flag, src}
#        }
#    }
#
#    #[derive(Debug)]
#    enum Instruction {
#        Mov { src: SourceOperand, dst: u8 },
#        Push { src: SourceOperand},
#        Pop { dst: u8 },
#        Jmp { label: String },
#        Cj { a: SourceOperand, b: SourceOperand, label: String, inst: u8 },
#        Add { a: SourceOperand, b: SourceOperand, dst: u8 },
#        Sub { a: SourceOperand, b: SourceOperand, dst: u8 },
#        And { a: SourceOperand, b: SourceOperand, dst: u8 },
#        Or { a: SourceOperand, b: SourceOperand, dst: u8 },
#        Not { a: SourceOperand, dst: u8 },
#        Xor { a: SourceOperand, b: SourceOperand, dst: u8 },
#        Nand { a: SourceOperand, b: SourceOperand, dst: u8 },
#        Nor { a: SourceOperand, b: SourceOperand, dst: u8 },
#        Div { a: SourceOperand, b: SourceOperand, dst: u8 },
#        Call { label: String },
#        Ret,
#    }
#
#    #[derive(Debug)]
#    struct EncodedInstruction {
#        opcode: u8,
#        arg1: u8,
#        arg2: u8,
#        result: u8,
#    }
#
#    enum AluOp {
#        Add,
#        Sub,
#        And,
#        Or,
#        Xor,
#        Nand,
#        Nor,
#        Div,
#    }
#
#    #[derive(Clone, Copy, PartialEq)]
#    enum Section {
#        Text,
#        Data,
#    }
#
#    /*
#    jump_start:
#        # comment
#        MOV 1, r3 # comment
#        MOV 2, ram
#        MOV 3, pc
#        MOV 4, io
#
#        MOV r1, r2
#        MOV ram, r2
#        MOV ram, ram
#        MOV ram, pc
#        MOV pc, ram
#        MOV io, ram
#        MOV io, pc
#        MOV io, io
#
#        PUSH r0
#        PUSH 255
#        PUSH ram
#        PUSH io
#        PUSH pc
#        POP r0
#        POP ram
#        POP io
#        POP pc
#
#        ADD 5, r2, r3
#        ADD r1, r2, r3
#
#        SUB 5, r2, r3
#        SUB r1, r2, r3
#
#        AND 5, r2, r3
#        AND r1, r2, r3
#
#        OR 5, r2, r3
#        OR r1, r2, r3
#
#        NOT 5, r3
#        NOT r1, r3
#
#        XOR 5, r2, r3
#        XOR r1, r2, r3
#
#        NAND 5, r2, r3
#        NAND r1, r2, r3 
#
#        NOR 5, r2, r3
#        NOR r1, r2, r3 
#
#        DIV 5, r2, r3
#        DIV r1, r2, r3 
#
#        CALL jump_here
#
#        CJE 5, 4, jump_start
#        CJNE 5, 4, jump_start
#        CJL 5, 4, jump_start
#        CJLE 5, 4, jump_start
#        CJG 5, 4, jump_start
#        CJGE 5, 4, jump_start
#
#        JMP jump_start
#
#    jump_here:
#        NOR 5, r2, r3
#        NOR r1, r2, r3  
#        RET
#    */
#
#    pub fn assembly(input: &str, output_debug: bool) {
#        const INSTRUCTION_BIT_DEPTH: u8 = 4;
#    
#        let lines: Vec<&str> = input.lines().collect();
#        let mut data_values: HashMap<String, u8> = HashMap::new();
#
#        // PASS 1: collect labels
#        let mut labels = HashMap::new();
#        let mut address: u8 = 0;
#        let mut last_label: Option<String> = None;
#
#        let mut section = Section::Text;
#        let mut text_end: u8 = 0;
#        let mut data_org: Option<u8> = None;
#
#        for line in &lines {
#            match parse_line(line) {
#                ParsedLine::Label(name) => {
#                    labels.insert(name.clone(), address);
#                    last_label = Some(name);
#                }
#                ParsedLine::Instruction(_) => {
#                    address += INSTRUCTION_BIT_DEPTH;
#                }
#                ParsedLine::Directive(d) => {
#                    match d {
#                        Directive::Org(addr) => {
#                            if section == Section::Data {
#                                data_org = Some(addr);
#
#                                if addr < text_end {
#                                    panic!("Error: data section overlaps with text section");
#                                }
#                            }
#                            address = addr;
#                        }
#                        Directive::Byte(value) => {
#                            if section == Section::Data {
#                                if let Some(label) = &last_label {
#                                    data_values.insert(label.clone(), value);
#                                } else {
#                                    panic!(".byte without label in data section");
#                                }
#                            }
#                            address += 1;
#                        }
#                        Directive::Text => {
#                            section = Section::Text;
#                        }
#                        Directive::Data => {
#                            section = Section::Data;
#                            // фиксируем конец текста
#                            text_end = address;
#                            let data_start = match data_org {
#                                Some(addr) => addr,
#                                None => text_end,
#                            };
#                            if data_start < text_end {
#                                panic!("Error: data section overlaps with text section");
#                            }
#                            address = data_start;
#                        }
#                    }
#                }
#                ParsedLine::Empty => {}
#            }
#        }
#
#        // Debug label addresses
#        if output_debug {
#            println!("# label addresses:");
#            for (key, value) in &labels {
#                println!("# {}: {:03}", key, value); 
#            }
#            println!("# ------------------------------\n");
#        }
#        
#        // PASS 2: encode instructions
#        let mut output: Vec<u8> = vec![0; 256];
#        let mut max_address: u8 = 0;
#        address = 0;
#
#        let mut section = Section::Text;
#        let mut text_end: u8 = 0;
#        let mut data_org: Option<u8> = None;
#    
#        for line in &lines {
#            match parse_line(line) {
#                ParsedLine::Instruction(instr) => {
#                    let encoded = encode_instruction(instr, &labels, &data_values);
#
#                    output[address as usize] = encoded.opcode;
#                    output[(address + 1) as usize] = encoded.arg1;
#                    output[(address + 2) as usize] = encoded.arg2;
#                    output[(address + 3) as usize] = encoded.result;
#
#                    address += INSTRUCTION_BIT_DEPTH;
#                    max_address = max_address.max(address);
#                }
#
#                ParsedLine::Directive(d) => {
#                    match d {
#                        Directive::Org(addr) => {
#                            if section == Section::Data {
#                                data_org = Some(addr);
#                            }
#                            address = addr;
#                        }
#                        Directive::Byte(value) => {
#                            output[address as usize] = value;
#                            address += 1;
#                            max_address = max_address.max(address);
#                        }
#                        Directive::Text => {
#                            section = Section::Text;
#                        }
#                        Directive::Data => {
#                            section = Section::Data;
#                            text_end = address;
#                            let data_start = match data_org {
#                                Some(addr) => addr,
#                                None => text_end,
#                            };
#                            if data_start < text_end {
#                                panic!("Error: data section overlaps with text section");
#                            }
#                            address = data_start;
#                        }
#                    }
#                }
#                ParsedLine::Label(_) | ParsedLine::Empty => {}
#            }
#        }
#
#        let used = max_address as usize;
#
#        // show assembly code
#        if !output_debug {
#            for  (chunk_idx, inst) in output[..used].chunks(INSTRUCTION_BIT_DEPTH as usize).enumerate(){
#                if inst.len() < 4 {
#                    break;
#                }
#                println!("0b{:08b}", inst[0]);
#                println!("0b{:08b}", inst[1]);
#                println!("0b{:08b}", inst[2]);
#                println!("0b{:08b}", inst[3]); 
#                println!();
#            }
#        }
#
#        // Debug
#        if output_debug {
#            println!("# bytes    |:addr|text instruction");
#            for  (chunk_idx, inst) in output[..used].chunks(INSTRUCTION_BIT_DEPTH as usize).enumerate(){
#                if inst.len() < 4 {
#                    break;
#                }
#                let base_addr = chunk_idx * INSTRUCTION_BIT_DEPTH as usize;
#
#                let text_inst = decode(inst[0], inst[1], inst[2], inst[3]);
#
#                println!("0b{:08b} #:{:03} |{}", inst[0],base_addr,text_inst);
#                println!("0b{:08b} #:{:03}", inst[1],base_addr+1);
#                println!("0b{:08b} #:{:03}", inst[2],base_addr+2);
#                println!("0b{:08b} #:{:03}", inst[3],base_addr+3); 
#                println!();
#            }
#        }
#    }
#
#    fn strip_comment(line: &str) -> &str {
#        if let Some(pos) = line.find('#') {
#            &line[..pos]
#        } else {
#            line
#        }
#    }
#
#    fn parse_line(line: &str) -> ParsedLine {
#        let line = line.trim();
#        let code = strip_comment(line).trim();
#
#        if code.is_empty() {
#            return ParsedLine::Empty;
#        }
#
#        if code.ends_with(':') {
#            let label = code.trim_end_matches(':').to_string();
#            return ParsedLine::Label(label);
#        }
#
#        let tokens: Vec<&str> = code
#            .split([' ', ','])
#            .filter(|s| !s.is_empty())
#            .collect();
#
#        if tokens.is_empty() {
#            return ParsedLine::Empty;
#        }
#
#        match tokens[0] {
#            ".data" => ParsedLine::Directive(Directive::Data),
#            ".text" => ParsedLine::Directive(Directive::Text),
#            ".org" => {
#                let value = parse_number(tokens[1]);
#                ParsedLine::Directive(Directive::Org(value))
#            }
#            ".byte" => {
#                let value = parse_number(tokens[1]);
#                ParsedLine::Directive(Directive::Byte(value))
#            },
#            "MOV" => parse_mov(&tokens),
#            "PUSH" => parse_push(&tokens),
#            "POP" => parse_pop(&tokens),
#            "JMP" => {
#                let label = tokens[1].to_string();
#                ParsedLine::Instruction(Instruction::Jmp { label })
#            },
#            "CJE"|"CJNE"|"CJL"|"CJLE"|"CJG"|"CJGE" => parse_cj(&tokens),
#            "ADD" => parse_alu(&tokens, AluOp::Add),
#            "SUB" => parse_alu(&tokens, AluOp::Sub),
#            "AND" => parse_alu(&tokens, AluOp::And),
#            "OR" => parse_alu(&tokens, AluOp::Or),
#            "NOT" => parse_not(&tokens),
#            "XOR" => parse_alu(&tokens, AluOp::Xor),
#            "NAND" => parse_alu(&tokens, AluOp::Nand),
#            "NOR" => parse_alu(&tokens, AluOp::Nor),
#            "DIV" => parse_alu(&tokens, AluOp::Div),
#            "CALL" => {
#                let label = tokens[1].to_string();
#                ParsedLine::Instruction(Instruction::Call { label })
#            },
#            "RET" => ParsedLine::Instruction(Instruction::Ret),
#            _ => panic!("Unknown instruction: {}", tokens[0]),
#        }
#    }
#    
#    // MOV: 
#    // * src регистров/RAM/INPUT/PC/Immediate Value 
#    // * dest регистр/RAM/OUTPUT/PC
#    fn parse_mov(tokens: &[&str]) -> ParsedLine {
#        let src = parse_operand(tokens[1], 128);
#        let dst = parse_source( tokens[2]);
#        ParsedLine::Instruction(Instruction::Mov {
#            src,
#            dst
#        })
#    }
#
#    fn parse_push(tokens: &[&str]) -> ParsedLine {
#        let src = parse_operand(tokens[1], 128);
#        ParsedLine::Instruction(Instruction::Push {src})
#    }
#
#    fn parse_pop(tokens: &[&str]) -> ParsedLine {
#        let dst= {
#            match tokens[1] {
#                "r0" | "r1" | "r2" | "r3" | "r4" | "r5" |
#                "pc" | "io" | "ram" => {
#                    parse_source(tokens[1])
#                }
#                _ => {
#                    panic!("Unknown source: {}", tokens[1]);
#                }
#            }
#        };
#        ParsedLine::Instruction(Instruction::Pop {
#            dst, 
#        })
#    }
#    
#    fn parse_cj(tokens: &[&str]) -> ParsedLine {
#        let a = parse_operand(tokens[1], 128);
#        let b = parse_operand(tokens[2], 64);
#        let label = tokens[3].to_string();
#        let inst = {
#            let inst = tokens[0];
#            match inst {
#                "CJE" => {0b00100000}, // 32 IF_EQUAL
#                "CJNE" => {0b00100001},// 33 IF_NOT_EQUAL
#                "CJL" => {0b00100010}, // 34 IF_LESS
#                "CJLE" => {0b00100011},// 35 IF_LESS_OR_EQUAL 
#                "CJG" => {0b00100100}, // 36 IF_GREATER
#                "CJGE" => {0b00100101},// 37 IF_GREATER_OR_EQUAL
#                _ => panic!("Unknown command:{inst}")
#            }
#        };
#        ParsedLine::Instruction(Instruction::Cj {a, b, label, inst })
#    }
#
#    fn parse_alu(tokens: &[&str], kind: AluOp) -> ParsedLine {
#        let a = parse_operand(tokens[1], 128);
#        let b = parse_operand(tokens[2], 64);
#        let dst = parse_source(tokens[3]);
#        match kind{
#            AluOp::Add => {ParsedLine::Instruction(Instruction::Add {a, b, dst })},
#            AluOp::Sub => {ParsedLine::Instruction(Instruction::Sub {a, b, dst })},
#            AluOp::And => {ParsedLine::Instruction(Instruction::And { a, b, dst })},
#            AluOp::Or => {ParsedLine::Instruction(Instruction::Or {a, b, dst })},
#            AluOp::Xor => {ParsedLine::Instruction(Instruction::Xor {a, b, dst })},
#            AluOp::Nand => {ParsedLine::Instruction(Instruction::Nand {a, b, dst })},
#            AluOp::Nor => {ParsedLine::Instruction(Instruction::Nor {a, b, dst })},
#            AluOp::Div => {ParsedLine::Instruction(Instruction::Div {a, b, dst })},
#        }
#    }
#
#    fn parse_not(tokens: &[&str]) -> ParsedLine {
#        let a = parse_operand(tokens[1], 128);
#        let dst = parse_source(tokens[2]);
#        ParsedLine::Instruction(Instruction::Not {a, dst })
#    }
#
#    fn parse_operand(s: &str, imm_flag: u8) -> SourceOperand {
#        // [label] — адрес
#        if s.starts_with('[') && s.ends_with(']') {
#            let label = &s[1..s.len() - 1];
#            return SourceOperand::new(imm_flag, Operand::IndirectAddr(label.to_string()));
#        }
#
#        // регистр
#        match s {
#            "r0" | "r1" | "r2" | "r3" | "r4" | "r5" |
#            "pc" | "io" | "ram" => {
#                return SourceOperand::new(0, Operand::Source(parse_source(s)));
#            }
#            _ => {}
#        }
#
#        // число
#        if let Ok(num) = s.parse::<u8>() {
#            return SourceOperand::new(imm_flag, Operand::Immediate(num));
#        }
#
#        // иначе это метка с константой
#        SourceOperand::new(imm_flag, Operand::MemLabel(s.to_string()))
#    }
#
#    fn parse_source(name: &str) -> u8 {
#        match name {
#            "r0" => 0,
#            "r1" => 1,
#            "r2" => 2,
#            "r3" => 3,
#            "r4" => 4,
#            "r5" => 5,
#            "pc" => 6,
#            "io" => 7,
#            "ram" => 8,
#            _ => panic!("Unknown register: {}", name),
#        }
#    }
#
#    fn parse_number(s: &str) -> u8 {
#        s.parse::<u8>().expect("Invalid number")
#    }
#
#    fn encode_src(src: SourceOperand, labels: &HashMap<String, u8>, data_values: &HashMap<String, u8>) -> u8 {
#        match src.src {
#            Operand::Source(s) => {s},
#            Operand::Immediate(v) => {v},
#            Operand::IndirectAddr(label) => {
#                *labels.get(&label).expect("Unknown label")  
#            } 
#            Operand::MemLabel(label) => {
#            // Если есть значение в data_values - берем его как immediate
#                if let Some(val) = data_values.get(&label) {
#                    *val
#                } else {
#                    // иначе используем адрес (если, например, jump)
#                    *labels.get(&label).expect("Unknown label")
#                } 
#            }                 
#        }
#    }
#
#    fn encode_instruction(
#        instr: Instruction,
#        labels: &HashMap<String, u8>,
#        data_values: &HashMap<String, u8>
#    ) -> EncodedInstruction {
#        match instr {
#            Instruction::Mov { src, dst } => EncodedInstruction {
#                opcode: src.imm_flag|0b00001100,  
#                arg1: encode_src(src, labels, data_values),
#                arg2: 0,
#                result: dst,
#            },
#            Instruction::Push { src } => EncodedInstruction {
#                opcode: src.imm_flag|0b00010111,  
#                arg1: encode_src(src, labels, data_values),
#                arg2: 0,
#                result: 0b00001000,
#            },
#            Instruction::Pop { dst } => EncodedInstruction {
#                opcode: 0b00010101,  
#                arg1: 0,
#                arg2: 0,
#                result: dst,
#            },
#            Instruction::Add { a, b, dst } => EncodedInstruction {
#                opcode: a.imm_flag|b.imm_flag,
#                arg1: encode_src(a, labels, data_values),
#                arg2: encode_src(b, labels, data_values),
#                result: dst,
#            },
#            Instruction::Sub { a, b, dst } => EncodedInstruction {
#                opcode: a.imm_flag|b.imm_flag|0b00000001,
#                arg1: encode_src(a, labels, data_values),
#                arg2: encode_src(b, labels, data_values),
#                result: dst,
#            },   
#            Instruction::And { a, b, dst } => EncodedInstruction {
#                opcode: a.imm_flag|b.imm_flag|0b00000010,
#                arg1: encode_src(a, labels, data_values),
#                arg2: encode_src(b, labels, data_values),
#                result: dst,
#            },  
#            Instruction::Or { a, b, dst } => EncodedInstruction {
#                opcode: a.imm_flag|b.imm_flag|0b00000011,
#                arg1: encode_src(a, labels, data_values),
#                arg2: encode_src(b, labels, data_values),
#                result: dst,
#            },  
#            Instruction::Not { a,dst } => EncodedInstruction {
#                opcode: a.imm_flag|0b00000100,
#                arg1: encode_src(a, labels, data_values),
#                arg2: 0,
#                result: dst,
#            },  
#            Instruction::Xor { a, b, dst } => EncodedInstruction {
#                opcode: a.imm_flag|b.imm_flag|0b00000101,
#                arg1: encode_src(a, labels, data_values),
#                arg2: encode_src(b, labels, data_values),
#                result: dst,
#            },  
#            Instruction::Nand { a, b, dst } => EncodedInstruction {
#                opcode: a.imm_flag|b.imm_flag|0b00000110,
#                arg1: encode_src(a, labels, data_values),
#                arg2: encode_src(b, labels, data_values),
#                result: dst,
#            },  
#            Instruction::Nor { a, b, dst } => EncodedInstruction {
#                opcode: a.imm_flag|b.imm_flag|0b00000111,
#                arg1: encode_src(a, labels, data_values),
#                arg2: encode_src(b, labels, data_values),
#                result: dst,
#            },  
#            Instruction::Div { a, b, dst } => EncodedInstruction {
#                opcode: a.imm_flag|b.imm_flag|0b00010100,
#                arg1: encode_src(a, labels, data_values),
#                arg2: encode_src(b, labels, data_values),
#                result: dst,
#            },  
#            Instruction::Call { label} => {
#                let addr = *labels
#                    .get(&label)
#                    .expect("Unknown label");
#
#                EncodedInstruction {
#                    opcode: 0b00001000,
#                    arg1: 0,
#                    arg2: 0,
#                    result: addr,
#                }
#            },
#            Instruction::Jmp { label } => {
#                let addr = *labels
#                    .get(&label)
#                    .expect("Unknown label");
#
#                EncodedInstruction {
#                    opcode: 0b10001100,
#                    arg1: addr,
#                    arg2: 0,
#                    result: 0b00000110,
#                }
#            },
#            Instruction::Cj { a, b, label, inst} => {
#                let addr = *labels
#                    .get(&label)
#                    .expect("Unknown label");
#
#                EncodedInstruction {
#                    opcode:  a.imm_flag|b.imm_flag|inst|0b00100000,
#                    arg1: encode_src(a, labels, data_values),
#                    arg2: encode_src(b, labels, data_values),
#                    result: addr,
#                }
#            },
#            Instruction::Ret => EncodedInstruction {
#                opcode: 0b00010000,
#                arg1: 0,
#                arg2: 0,
#                result: 0,
#            },
#        }
#    }
#
#    pub mod disassembly {
#        fn reg_name(id: u8) -> &'static str {
#            match id {
#                0 => "r0",
#                1 => "r1",
#                2 => "r2",
#                3 => "r3",
#                4 => "r4",
#                5 => "r5",
#                6 => "pc",
#                7 => "io",
#                8 => "ram",
#                _ => "unk",
#            }
#        }
#
#        fn decode_operand(flag: bool, value: u8) -> String {
#            if flag {
#                value.to_string()
#            } else {
#                reg_name(value).to_string()
#            }
#        }
#
#        pub fn decode(opcode: u8, arg1: u8, arg2: u8, result: u8) -> String {
#            let ai = opcode & 0b10000000 != 0;
#            let bi = opcode & 0b01000000 != 0;
#            let base = opcode & 0b00111111;
#
#            let a = decode_operand(ai, arg1);
#            let b = decode_operand(bi, arg2);
#            let dst = reg_name(result);
#
#            match base {
#                0b00000000 => format!("ADD {}, {}, {}", a, b, dst),
#                0b00000001 => format!("SUB {}, {}, {}", a, b, dst),
#                0b00000010 => format!("AND {}, {}, {}", a, b, dst),
#                0b00000011 => format!("OR {}, {}, {}", a, b, dst),
#                0b00000100 => format!("NOT {}, {}", a, dst),
#                0b00000101 => format!("XOR {}, {}, {}", a, b, dst),
#                0b00000110 => format!("NAND {}, {}, {}", a, b, dst),
#                0b00000111 => format!("NOR {}, {}, {}", a, b, dst),
#                0b00001100 => {
#                    let src = decode_operand(ai, arg1);
#                    let dst = reg_name(result);
#                    format!("MOV {}, {}", src, dst)
#                },
#                0b00010111 => {
#                    let src = decode_operand(ai, arg1);
#                    format!("PUSH :{}", src)
#                },
#                0b00010101 => {
#                    let dst = reg_name(result);
#                    format!("POP {}", dst)
#                },
#                0b00010100 => format!("DIV {}, {}, {}", a, b, dst),
#                0b00001000 => {
#                    format!("CALL :{:03}", result)
#                },
#                0b00010000 => "RET".to_string(),
#                0b00100000 => format!("CJE {}, {}, :{:03}", a, b, result),
#                0b00100001 => format!("CJNE {}, {}, :{:03}", a, b, result),
#                0b00100010 => format!("CJL {}, {}, :{:03}", a, b, result),
#                0b00100011 => format!("CJLE {}, {}, :{:03}", a, b, result),
#                0b00100100 => format!("CJG {}, {}, :{:03}", a, b, result),
#                0b00100101 => format!("CJGE {}, {}, :{:03}", a, b, result),
#                _ => format!(
#                    "Unknown opcode={:08b} arg1={} arg2={} dst={:08}",
#                    opcode, arg1, arg2, result
#                ),
#            }
#        }
#    }
# }