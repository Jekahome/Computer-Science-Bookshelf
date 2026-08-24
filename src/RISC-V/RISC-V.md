# Однотактный (Single-cycle) процессор на архитектуре RV32I (базовый 32-битный RISC-V)
(Однотактный - одна инструкция выполняется ровно за один такт таймера (clk))

Семейство RISC-V — Reduced Instruction Set Computer (компьютер с сокращенным набором инструкций 5-й версии), имеет различные название архитектуры набора команд (ISA, Instruction Set Architecture) но базовая это RV32I.

Буква «I» (Integer) в RV32I означает: базовый целочисленный набор инструкций (Base Integer Instruction Set).

RV32I это обязательное «ядро» любого процессора RISC-V, который обязан уметь выполнять и ADD (R-тип), и ADDI (I-тип), и SW (S-тип), и BNE (B-тип). Существуют и другие базовые наборы, например, целочисленный упрощенный RV32E (для микроконтроллеров с 16 регистрами вместо 32).

К этой базовой части «I» могут добавляться стандартные расширения (буквы):
* M — аппаратное умножение и деление (Multiply)
* A — атомарные инструкции (Atomic)
* F / D — работа с плавающей точкой одинарной/двойной точности (Float / Double)
* C — сжатые 16-битные инструкции (Compressed)
* Zicsr — инструкции работы с CSR
* Zifencei — инструкция FENCE.I

Если процессор поддерживает базовый набор, умножение и плавающую точку, его называют, например, RV32IMF.

Однотактный процессор хорошо подходит для понимания устройства микроархитектуры, но их частоту невозможно поднять высоко из-за медленных инструкций вроде `lw`, у которых слишком долгая суммарная задержка распространения (propagation delay). Именно эта проблема и привела инженеров к изобретению конвейера (pipelining), где этот длинный путь просто разрезали на 5 независимых коротких тактов.


#### Микрокод

В раннюю эпоху компьютеров, память была очень дорогой и медленной, поэтому архитектуры вроде x86 (CISC) создавали так, чтобы одна инструкция делала как можно больше работы.

RISC (сокращенный набор) предложил противоположную идею: сделать все инструкции предельно простыми, одинаковыми по длине и выполнять работу с памятью только через отдельные команды LOAD и STORE. Что привело к такому преимуществу:
* Простая и быстрая аппаратная декодировка инструкций
* Простота использования конвейера (Pipelining)
* Меньше транзисторов на кристалле и ниже энергопотребление

Преимущества подхода RISC сподвигли производителей на изменение CISC-архитектур. Но так как под CISC уже было написано огромное количество софта, инженерам пришлось идти на технический компромисс. Снаружи (для программиста и ОС) процессор остается чистым x86 (CISC) со всеми его сложными инструкциями переменной длины, но внутри (на кристалле) его превратили в быстрый RISC-движок.


```
[ Сложный код x86 ] 
        │
        ▼
┌─────────────────────────┐
│ Hardware Decoders / ROM │ ◄── Декодируют x86 в микрооперации (μops)
└─────────────────────────┘
        │
        ▼
[ Простые RISC-подобные μops (40-64 бита) ]
        │
        ▼
┌─────────────────────────┐
│ Out-of-Order RISC Core  │ ◄── Быстрый RISC-конвейер исполняет μops
└─────────────────────────┘

```

Для реализации этой идеи применили концепцию **микрокода**.
* Простые команды x86 (например, add eax, ebx) аппаратно «на лету» декодируются в 1 микрооперацию ($\mu op$), которая ничем не отличается от инструкции RISC.
* Сложные команды x86 (например, rep movsb — скопировать блок памяти) аппаратно передаются во внутреннюю память микрокода (Microcode ROM). Микрокод «скармливает» конвейеру цепочку из десятков простых RISC-подобных микроопераций ($\mu ops$).

Таким образом, современный процессор Intel Core или AMD Ryzen — это RISC-процессор, у которого на входе стоит аппаратный транслятор с языка x86.Наличие слоя микроопераций и микрокода вносит дополнительную сложность для понимания и глубокой оптимизации программ: процессор исполняет не ваши исходные инструкции, а сгенерированные микрооперации ($\mu ops$).

> [!IMPORTANT]
> RISC-V создавался так, чтобы микрокод был не нужен.

#### Симуляторы

[Симулятор ассемблера Venus](https://venus.kvakil.me/) – это веб-симулятор ассемблера RISC-V

Симулятор схем [Digital (Digital Logic Designer от Helmut Neemann)](https://github.com/hneemann/Digital)

## Содержание


* [Декодирование инструкции](#Декодирование-инструкции)
* [Von Neumann vs Harvard architecture](#von-neumann-vs-harvard-architecture)
* [Регистры](#Регистры)
* [Типы инструкций](#Типы-инструкций)
* [Инструкции I-типа](#Инструкции-i-типа)
    * [Группа: Арифметика](#Группа-Арифметика)
    * [Группа: Load](#Группа-load)
    * [Группа: JALR](#Группа-jalr)
* [Инструкции System](#Инструкции-system)
* [Инструкции Control](#Инструкции-control)
* [Инструкции R-типа](#Инструкции-r-типа)
* [Инструкции S-типа](#Инструкции-s-типа)
* [Инструкции B-типа](#Инструкции-b-типа)
* [Инструкции U-типа](#Инструкции-u-типа)
* [Инструкции J-типа](#Инструкции-j-типа)
* [Расширение Zicsr (CSR)](#zicsr-расширение-control-and-status-registers-csr)
    * Модуль: CSR Unit
* [Расширение M](#Расширение-m)    
* [Псевдоинструкции](#Псевдоинструкции)    
* [Модули](#Модули)    
* [Примеры](#Примеры)
* [Схемы, код и данные](#Схемы-код-и-данные)

---
 

## Декодирование инструкции

Online instruction decoder:
* [rvcodec.js](https://luplab.gitlab.io/rvcodecjs/?utm_source=chatgpt.com#q=addi&abi=false&isa=RV32I) 
* [RISC-V Instruction Decoder](https://rvopcode.com/en/instruction-decoder?utm_source=chatgpt.com)

[Bin to Hex](https://www.binaryhexconverter.com/binary-to-hex-converter):
```rust,editable
fn main() {
    let binary = "00000000001001010010001000010011";

    let value = u32::from_str_radix(binary, 2).unwrap();

    println!("0x{:08X}", value);
}
```

Hex Instruction decode:
```rust,editable,abraetablemini
fn main() {
  let instr: u32 = 0x00208463; // Hexadecimal
      
  println!("Instr hex    : 0x{:08x}", instr);      
  decode(instr);
}
















# #[derive(Debug)]
# enum InstFormat {
#    R,
#    I,
#    S,
#    B,
#    U,
#    J,
#    SystemZicsr,
#    ControlZifencei,   
# }
#
# fn get_format(opcode: u32) -> Option<InstFormat> {
#    match opcode {
#        0b0110011 => Some(InstFormat::R),
#
#        0b0010011 | // I-type ALU
#        0b0000011 | // loads
#        0b1100111   // JALR
#            => Some(InstFormat::I),
#
#        0b0100011 => Some(InstFormat::S),
#        0b1100011 => Some(InstFormat::B),
#
#        0b0110111 | // LUI
#        0b0010111   // AUIPC
#            => Some(InstFormat::U),
#
#        0b1101111 => Some(InstFormat::J),
#        0b1110011 => Some(InstFormat::SystemZicsr), // System, CSR
#        0b0001111 => Some(InstFormat::ControlZifencei), // Control, Zifencei
#        
#        _ => None,
#    }
# }
#
# fn sign_extend(value: u32, bits: u32) -> i32 {
#    let shift = 32 - bits;
#    ((value << shift) as i32) >> shift
# }
#
# fn decode(instr: u32) {
#    let opcode: u32 = instr & 0x7F;
#
#
#    let format = match get_format(opcode) {
#        Some(f) => f,
#        None => {
#            println!("Unknown format");
#            return;
#        }
#    };
#
#    match format {
#        InstFormat::R => {
#            let rd     = (instr >> 7) & 0x1F;
#            let funct3 = (instr >> 12) & 0x07;
#            let rs1    = (instr >> 15) & 0x1F;
#            let rs2    = (instr >> 20) & 0x1F;
#            let funct7 = (instr >> 25) & 0x7F;
#            let r_type = r_type_name(funct3, funct7);
#            
#            println!("Assembly     : {}", assembly_r(r_type, rd, rs1, rs2));
#            println!("Format       : R {}",if funct7 == 0b0000001 {"(M)"}else{""});
#            println!("Instr        : {:032b}", instr);
#            println!("opcode       : .........................{:07b} (0x{:x})", opcode, opcode);
#            println!("rd (receiver): ....................{:05b}....... x{} ({:?})", rd, rd, REG_ALIASES[rd as usize]);
#            println!("funct3       : .................{:03b}............ ({})", funct3, r_type);
#            println!("rs1 (source) : ............{:05b}............... x{} ({:?})", rs1, rs1, REG_ALIASES[rs1 as usize]);
#            println!("rs2 (source) : .......{:05b}.................... x{} ({:?})", rs2, rs2, REG_ALIASES[rs2 as usize]);
#            println!("funct7       : {:07b}......................... ({})", funct7, funct7);
#        }
#
#        InstFormat::I => {
#            let rd:u32     = (instr >> 7) & 0x1F;
#            let funct3:u32  = (instr >> 12) & 0x07;
#            let funct7:u32  = (instr >> 25) & 0x7F;
#            let rs1:u32     = (instr >> 15) & 0x1F;
#             
#            let imm_raw:u32  = if opcode == 0b0010011 && (funct3 == 0b001 || funct3 == 0b101) {
#                  (instr >> 20) & 0x1F
#            } else {
#                  (instr >> 20) & 0xFFF
#            };
#            let imm_sext = if opcode == 0b0010011 && (funct3 == 0b001 || funct3 == 0b101) {
#                imm_raw as i32
#            } else {
#                sign_extend(imm_raw, 12)
#            };
#
#            let i_format = match opcode {
#               0b0010011 => "ALU immediate",
#               0b0000011 => "LOAD",
#               0b1100111 => "JALR",
#               _ => "Unknown format" 
#            };
#            let i_type = i_type_name(opcode, funct3, funct7);
#            println!("Assembly     : {}", assembly_i(opcode, i_type, rd, rs1, imm_raw));
#            println!("Format       : I ({i_format})");
#            println!("instr        : {:032b}", instr);
#            println!("opcode       : .........................{:07b} (0x{:x})", opcode, opcode);
#            println!("rd (receiver): ....................{:05b}....... x{} ({:?})", rd, rd, REG_ALIASES[rd as usize]);
#            println!("funct3       : .................{:03b}............ ({})", funct3, i_type);
#            println!("rs1 (source) : ............{:05b}............... x{} ({:?})", rs1, rs1, REG_ALIASES[rs1 as usize]);
#            if opcode == 0b0010011 && (funct3 == 0b001 || funct3 == 0b101) {
#                println!("imm shamt    : .......{:05b}.................... ({})", imm_raw, imm_raw);
#                println!("funct7       : {:07b}......................... ({})", funct7, funct7);
#            } else {
#                println!("imm          : {:012b}.................... raw=0x{:X} signed={} decimal", imm_raw, imm_raw, imm_sext);
#            }
#        }
#
#        InstFormat::S => {
#            let funct3:u32 = (instr >> 12) & 0x07;//сдвиг вправо на 12 позиций и применить маску 00000111 для трех младших бит
#            let rs1    = (instr >> 15) & 0x1F;// маска 11111
#            let rs2    = (instr >> 20) & 0x1F;
#
#            let imm_raw_2 = (instr >> 25) & 0x7F;
#            let imm_raw_1 = (instr >> 7)  & 0x1F;
#            let imm_raw = (imm_raw_2 << 5) | imm_raw_1;// сдвигаем на 5 знаков влево, чтобы корректно проставить разрядность на 12 бит
#
#            let imm_sext = sign_extend(imm_raw, 12);
#            let s_type = s_type_name(funct3);
#            println!("Assembly     : {}", assembly_s(s_type, rs1, rs2, imm_raw));
#            println!("Format       : S");
#            println!("instr        : {:032b}", instr);
#            println!("opcode       : .........................{:07b}", opcode);
#            println!("imm          : {:07b}.............{:05b}....... 0b{:012b} raw=0x{:03X} signed={} decimal", imm_raw_2, imm_raw_1, imm_raw, imm_raw, imm_sext);
#            println!("funct3       : .................{:03b}............ ({})", funct3, s_type);
#            println!("rs1 (source) : ............{:05b}............... x{} ({:?})", rs1, rs1, REG_ALIASES[rs1 as usize]);
#            println!("rs2 (source) : .......{:05b}.................... x{} ({:?})", rs2, rs2, REG_ALIASES[rs2 as usize]);
#        }
#
#        InstFormat::B => {
#            let rs1 = (instr >> 15) & 0x1F;
#            let rs2 = (instr >> 20) & 0x1F;
#            let funct3 = (instr >> 12) & 0x07;
#
#            let imm_12    = (instr >> 31) & 0x1;
#            let imm_10_5  = (instr >> 25) & 0x3F;
#            let imm_4_1   = (instr >> 8)  & 0x0F;
#            let imm_11    = (instr >> 7)  & 0x1;
#            let imm_0     = 0;
#
#            let imm =
#              (imm_12   << 12) |
#              (imm_11   << 11) |
#              (imm_10_5 << 5)  |
#              (imm_4_1  << 1)  |
#              (imm_0);     
#
#             let imm_sext = sign_extend(imm, 13);
#             let b_type = b_type_name(funct3);
#             println!("Assembly     : {}", assembly_b(b_type, rs1, rs2, imm_sext));
#             println!("Format       : B");
#             println!("instr        : {:032b}", instr);
#             println!("opcode       : .........................{:07b}", opcode);
#             println!("imm[11]    c : ........................{:01b}.......", imm_11 );
#             println!("imm[4:1]   a : ....................{:04b}........", imm_4_1 );
#             println!("funct3       : .................{:03b}............ ({})", funct3, b_type);
#             println!("rs1 (source) : ............{:05b}............... x{} ({:?})", rs1, rs1, REG_ALIASES[rs1 as usize]);
#             println!("rs2 (source) : .......{:05b}.................... x{} ({:?})", rs2, rs2, REG_ALIASES[rs2 as usize]);
#             println!("imm[10:5]  b : .{:06b}.........................", imm_10_5 );
#             println!("imm[12]    d : {:01b}...............................", imm_12 );
#             println!(
#                      "imm label    : {:01b}{:01b}{:06b}{:04b}{:01b}................... 0x{:X}",
#              imm_12, 
#              imm_11,
#              imm_10_5, 
#              imm_4_1, 
#              imm_0, 
#              imm_sext );
#            println!("part imm     : dcbbbbbbaaaa0...................");
#        }
#
#        InstFormat::U => {
#            let rd  = (instr >> 7) & 0x1F;
#            let imm_raw = instr & 0xFFFFF000;// готовое 20-битное значение, сдвинутое в старшие биты
#             let imm20 = (instr >> 12) & 0xFFFFF;
# 
#             let u_type_name = match opcode {
#               0b0110111 => "lui",
#               0b0010111 => "auipc", 
#               
#               _ => "Unknown format" 
#            };
#              
#            println!("Assembly     : {}", assembly_u(u_type_name, rd,  imm_raw));
#            println!("Format       : U");
#            println!("instr        : {:032b}", instr);
#            println!("opcode       : .........................{:07b} ({})", opcode, u_type_name);
#            println!("rd (receiver): ....................{:05b}....... x{} ({:?})", rd, rd, REG_ALIASES[rd as usize]);
#            println!("imm          : {:020b}............ raw=0x{:03X}", imm20, imm_raw );
#        }
#
#        InstFormat::J => {
#            let rd = (instr >> 7) & 0x1F;
#
#            let imm_20    = (instr >> 31) & 0x1;
#            let imm_19_12 = (instr >> 12) & 0xFF;
#            let imm_11    = (instr >> 20) & 0x1;
#            let imm_10_1  = (instr >> 21) & 0x3FF;
#            let imm_0     = 0;
#
#            let imm =
#                (imm_20    << 20) |
#                (imm_19_12 << 12) |
#                (imm_11    << 11) |
#                (imm_10_1  << 1)  |
#                (imm_0);
#
#            let imm_sext = sign_extend(imm, 21);
#
#            println!("Assembly     : jal x{}, 0x{:X}", rd,  imm_sext);
#            println!("Format       : J");
#            println!("instr        : {:032b}", instr);
#            println!("opcode       : .........................{:07b}", opcode);
#            println!("rd (receiver): ....................{:05b}....... x{} ({:?})", rd, rd, REG_ALIASES[rd as usize]);
#             
#
#            println!("imm[19:12] c : ............{:08b}............", imm_19_12 );
#            println!("imm[11]    b : ...........{:01b}....................", imm_11 );
#            println!("imm[10:1]  a : .{:010b}.....................", imm_10_1);
#            println!("imm[20]    d : {:01b}...............................", imm_20 );
#
#            println!(
#                      "imm label    : {:01b}{:08b}{:01b}{:010b}{:01b}............ 0x{:X}",
#                imm_20,
#                imm_19_12,
#                imm_11,
#                imm_10_1,
#                imm_0,
#                imm_sext
#           );
#           println!("part imm     : dccccccccbaaaaaaaaaa0............");
#        }
#
#         InstFormat::SystemZicsr => {
#             let imm = (instr >> 20) & 0xFFF;
#             let funct3:u32  = (instr >> 12) & 0x07;
#             let sys_name = system_name(imm, funct3);
#              
#              if funct3 == 0b000 {
#                println!("Assembly     : {}", sys_name);
#                println!("Format       : I {}", if sys_name=="mret"{"(Privileged ISA)"}else{"(System)"});
#                println!("instr        : {:032b}", instr);
#                println!("opcode       : .........................{:07b}", opcode );
#                println!("funct3       : .................{:03b}............", funct3);
#                println!("imm          : {:012b}.................... raw=0x{:03X}", imm, imm );                
#              }else{
#
#                let rd:u32     = (instr >> 7) & 0x1F;
#                let rs1:u32     = (instr >> 15) & 0x1F;
#                let uimm = (instr >> 15) & 0x1F;
#                let csr_name = match imm {
#                    0x300 => "mstatus",
#                    0x304 => "mie",
#                    0x305 => "mtvec",
#                    0x340 => "mscratch",
#                    0x341 => "mepc",
#                    0x342 => "mcause",
#                    0x343 => "mtval",
#                    0x344 => "mip",
#                    0xC00 => "cycle",
#                    0xC80 => "cycleh",
#                    0xC01 => "mtime_lo",
#                    0xC81 => "mtime_hi",
#                    _ => "unknown csr register",
#                };
#
#                println!("Assembly     : {}", assembly_csr(sys_name, rd, rs1, uimm, csr_name));
#                println!("Format       : I (Zicsr)");
#                println!("instr        : {:032b}", instr);
#                println!("opcode       : .........................{:07b}", opcode );
#                println!("rd (receiver): ....................{:05b}....... x{} ({:?})", rd, rd, REG_ALIASES[rd as usize]);
#                println!("funct3       : .................{:03b}............ ({})", funct3, sys_name);
#                if sys_name == "csrrwi"{
#                  
#                  println!("uimm         : ............{:05b}............... 0x{:03X} ", uimm, uimm as u32);
#                }else{
#                  println!("rs1 (source) : ............{:05b}............... x{} ({:?})", rs1, rs1, REG_ALIASES[rs1 as usize]);
#                }
#                println!("addr csr     : {:012b}.................... 0x{:03X} ({})", imm, imm, csr_name );
#              }
#         }
#
#        InstFormat::ControlZifencei => {
#             let funct3 = (instr >> 12) & 0x07;
#            let control_name = match funct3 {
#               0b000 => "fence",
#               0b001 => "fence.і", 
#                _ => "unknown",  
#            };
#            println!("Assembly     : {}", control_name);
#            println!("Format       : I {}", if control_name == "fence.і"{"(Zifencei)"}else{""});
#            println!("instr        : {:032b}", instr);
#            println!("opcode       : .........................{:07b}", opcode );
#            println!("funct3       : .................{:03b}............ ({})", funct3, control_name);
#        }
#      }
# }
# 
# static REG_ALIASES: [&str; 32] = [
#    "zero", "ra", "sp", "gp", "tp",
#    "t0", "t1", "t2",
#    "s0/fp", "s1",
#    "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7",
#    "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
#    "t3", "t4", "t5", "t6",
# ];

fn system_name(imm: u32, funct3: u32) -> &'static str{

  match funct3 {
    0b000 => {
      match imm {
        0b000000000000 => "ecall",
        0b000000000001 => "ebreak", 
        0b001100000010 => "mret",
          _ => "unknown",  
      }     
    }
    0b001 => "csrrw",
    0b010 => "csrrs",
    0b011 => "csrrc",
    0b101 => "csrrwi",
    0b110 => "csrrsi",
    0b111 => "csrrci",
    _ => "unknown csr",

  }
}

fn i_type_name(opcode: u32, funct3: u32, funct7: u32 ) -> &'static str {
  match opcode {
    0b0010011 => { // "ALU immediate"
      match funct3 {
          0b000 => "addi",
          0b010 => "slti",
          0b011 => "sltiu",
          0b100 => "xori",
          0b110 => "ori",
          0b111 => "andi",
          0b001 => "slli",

          0b101 => {
              // различаем SRLI и SRAI через funct7
              match funct7 {
                  0b0000000 => "srli",
                  0b0100000 => "srai",
                  _ =>  unimplemented!("i_type_name ALU immediate UNKNOWN_SHIFT"),
              }
          }
          _ => unimplemented!("i_type_name ALU immediate")   
      }
    }
    0b0000011 => {// "Load"
      match funct3 {
        0b000 => "lb",
        0b001 => "lh",
        0b010 => "lw",
        0b100 => "lbu",
        0b101 => "lhu",
        _ => unimplemented!("i_type_name Load") 
      }
    },
    0b1100111 =>  {"jalr" } ,
    0b1110011 =>  {unimplemented!("System") /* "System" */ } ,
    _ => unimplemented!("i_type_name opcode") 
    
  }
}

fn r_type_name(funct3: u32, funct7: u32 ) -> &'static str {
  match funct3 {
    0b000 => {
      match funct7 {
        0b0000000 => "add",
        0b0100000 => "sub",
        0b0000001 => "mul",
        _ =>  unimplemented!("r_type_name unknown type funct7 for funct3 0b000"),
      }
    },
    0b001 => {
      match funct7 {
        0b0000000 => "sll",
        0b0000001 => "mulh",
       
        _ =>  unimplemented!("r_type_name unknown type funct7 for funct3 0b001"),
      }
    },
    0b010 => {
      match funct7 {
        0b0000000 => "slt",
        0b0000001 => "mulhsu",
        _ =>  unimplemented!("r_type_name unknown type funct7 for funct3 0b010"),
      }
    },
    0b011 => {
      match funct7 {
        0b0000000 => "sltu",
        0b0000001 => "mulhu",
        _ =>  unimplemented!("r_type_name unknown type funct7 for funct3 0b011"),
      }
    }, 
    0b100 => {
      match funct7 {
        0b0000000 => "xor",
        0b0000001 => "div",
        _ =>  unimplemented!("r_type_name unknown type funct7 for funct3 0b100"),
      }
    }, 
    0b101 => {
      match funct7 {
        0b0000000 => "srl",
        0b0100000 => "sra",
        0b0000001 => "divu",
        _ =>  unimplemented!("r_type_name unknown type funct7 for funct3 0b101"),
      }
    },
    0b110 => {
      match funct7 {
        0b0000000 => "or",
        0b0000001 => "rem",
        _ =>  unimplemented!("r_type_name unknown type funct7 for funct3 0b110"),
      }
    }, 
    0b111 => {
      match funct7 {
        0b0000000 => "and",
        0b0000001 => "remu",
        _ =>  unimplemented!("r_type_name unknown type funct7 for funct3 0b111"),
      }
    },  
    _ => unimplemented!("r_type_name unknown type") 
  }
}

fn s_type_name(funct3: u32 ) -> &'static str {
  match funct3 {
    0b000 => "sb",
    0b001 => "sh",
    0b010 => "sw",
    _ => unimplemented!("s_type_name Store") 
  }
}

fn b_type_name(funct3: u32 ) -> &'static str {
  match funct3 {
    0b000 => "beq",
    0b001 => "bne",
    0b100 => "blt",
    0b101 => "bge",
    0b110 => "bltu",
    0b111 => "bgeu",
    _ => unimplemented!("b_type_name Branch") 
  }
}

fn assembly_i(opcode: u32, i_type: &str, rd: u32 , rs1: u32 , imm_raw: u32 ) -> String{
  match opcode {
    0b0010011 => { // ALU immediate
      format!("{} x{}, x{}, 0x{:X}", i_type, rd, rs1, imm_raw)
    }
    0b0000011 => {// LOAD
      format!("{} x{}, 0x{:X}(x{})", i_type, rd, imm_raw, rs1)
    }
    0b1100111 => {// JALR
      format!("{} x{}, 0x{:X}(x{})", i_type, rd, imm_raw, rs1)
    }
    _ => unimplemented!("assembly_i opcode")
  }
}

fn assembly_r(r_type: &str, rd: u32 , rs1: u32 , rs2: u32 ) -> String{
  format!("{} x{}, x{}, x{}", r_type, rd, rs1, rs2)
}

fn assembly_s(s_type: &str, rs1: u32, rs2: u32, imm_raw: u32 ) -> String{
  format!("{} x{}, 0x{:X}(x{})", s_type, rs2, imm_raw, rs1)
}

fn assembly_u(u_type: &str, rd: u32,  imm_raw: u32 ) -> String{
  format!("{} x{}, 0x{:X}", u_type, rd, imm_raw)
}

fn assembly_b(b_type: &str, rs1: u32, rs2: u32 , imm_raw: i32 ) -> String{
  format!("{} x{}, x{}, 0x{:X}", b_type, rs1, rs2, imm_raw)
}

fn assembly_csr(sys_name: &str, rd: u32, rs1: u32, uimm: u32, csr_name: &str) -> String{
  if sys_name == "csrrwi" {
    format!("{} x{}, {}, 0x{:03X}", sys_name, rd, csr_name, uimm)
  }else{
    format!("{} x{}, {}, x{}", sys_name, rd, csr_name, rs1)
  }
}
```

---

## Von Neumann vs Harvard architecture

Стандарт ISA RISC-V не определяет как именно процессор должен его реализовать, поэтому можно реализовать как:
* Harvard architecture (Гарвардскую арх. с отдельной памятью инструкций и данных);
* Von Neumann architecture (фон-неймановскую арх. c общей памятью);

RISC-V — это load-store архитектура (как почти все современные). Компилятор и ассемблер не знают о нашей физической реализации. Они рассчитывают на следующую модель, где инструкции и данные в одной памяти т.е. **Von Neumann architecture**:
* Единое адресное пространство
* Инструкции и данные находятся в одной большой адресной памяти
* Программа может читать/писать данные по любому адресу с помощью lw/sw, lb/sb и т.д.
* Для инструкций используется Program Counter (PC)

```
          CPU
           |
 ROM Общая память (Program + Data)
```


> Принципы фон Неймана:
>
> * Двоичность: Все данные и команды кодируются с использованием двоичной системы (нули и единицы).
> * Принцип хранимой программы (программного управления): Программа загружается в память и выполняется процессором автоматически, шаг за шагом.
> * Принцип однородности памяти: И программы, и данные находятся в одном и том же запоминающем устройстве. Компьютер может выполнять операции с командами так же, как и с данными.
> * Принцип адресности: Память состоит из пронумерованных ячеек (адресов). Процессор может в любой момент обратиться к нужной ячейке для получения или записи информации.

 
Плюс при реализации Фон Неймановской арх. в том, что не нужно брать на себя роль операционной системы для распределения инструкций и данных по разным модулям памяти (через скрипты Linker Script, Startup). Но минус «узкое место фон Неймана» в том, что команды и данные хранятся вместе, система не может одновременно считывать код и обрабатывать информацию, нам придется реализовывать логику дозагрузку данных для инструкций требующих доступа к памяти, т.е. на первом такте парсим саму инструкцию и стопорим процессор, а на следующем такте снова обращаться к памяти за данными, в то время как в Гарвардской арх. это происходит одновременно: на шине инструкций (программы) мы получаем инструкцию, а на шине данных - данные.

 
Для учебных целей и удобства часто делают *чистую* **Harvard architecture** (разделённую память):
* Отдельная Instruction Memory (ROM) — только чтение инструкций
* Отдельная Data Memory (RAM) — для данных и стека

```
           CPU
          /   \
         /     \
 I-Memory       D-Memory
(ROM Program)   (RAM Data)
```

 
Выгодно реализовать Гарвардскую арх., но так как мы хотим запускать реальные скомпилированные программы, которые содержат и код и данные в одной файле это уже предполанает фон-неймановскую арх. Что бы вести себя как фон-неймановскую арх. но физически быть Гарвардской арх. нам необходимо разметить память используя скрипты Linker Script, Startup. И реализовать возможность чтения из ROM не только инструкций но и данных, так как необходимо скопировать секцию .data в RAM при старте.

> [!INFO]
> Операционная система делает всю эту работу:
> * Загружает программу в память
> * Копирует .data из файла в RAM
> * Инициализирует стек
> * Вызывает main()
> * Обрабатывает системные вызовы

Потребуется модуль *Address Decoder* который в зависимости от адреса, будет распределять доступ к нужному блоку (RAM/ROW/MMIO):
 
| Области памяти, Harvard architecture | Адресный диапазон    | Размер   | Что там лежит                         | Как компилятор это помечает    |
|-------------------------------|-----------------------------|----------|---------------------------------------|--------------------------------|
| **Instruction Memory (ROM)**  | 0x00000000 – 0x001FFFFF     | 2 MB     | Код программы + константы             | `.text` + `.rodata`            |
| **MMIO (периферия)**          | 0x10000000 – 0x1000FFFF     | 64 KB    | LEDs, UART, таймеры и т.д.            | Специальные адреса             |
| **Data Memory (RAM)**         | 0x80000000 – 0x807FFFFF     | 8 MB     | Стек, глобальные переменные, heap     | `.data`, `.bss`, стек          |
| **Video Memory**              | 0x90000000 – 0x900FFFFF     | 1 MB     | Видеопамять                           |                                |

```

|--------------------------|
|      Video Memory        |
| 2415919104 – 2416967679  |
| 0x90000000 – 0x900FFFFF  |
|--------------------------|
|                          |
|                          |
|--------------------------|
|     Data Memory (RAM)    |
| 2147483648 – 2155872255  |
| 0x80000000 – 0x807FFFFF  |
|--------------------------|
|                          |
|                          |
|--------------------------|
|     MMIO (периферия)     |
|  268435456 – 268500991   |
| 0x10000000 – 0x1000FFFF  |
|--------------------------|
|                          |
|                          |
|--------------------------|
| Instruction Memory (ROM) |
|          0 – 2097151     |
| 0x00000000 – 0x001FFFFF  |
|--------------------------|
```

Для запуска кода мы должны самостоятельно подготовить состояние памяти процессора:
* Файл **Startup.S** — это "входная точка" (аналог `_start`):
   - Настраивает стек (`sp`)
   - Копирует инициализированные данные (`.data`) из ROM в RAM
   - Обнуляет `.bss`
   - Вызывает `main()`
* Файл **linker.ld** это Linker script — говорит компоновщику:
   - Куда класть код (ROM)
   - Куда класть данные (RAM)
   - Какие символы (`__data_start`, `__data_load` и т.д.) создать

Startup код (минимум)
* Инициализирует указатель стека (sp = конец RAM)
* Копирует .data из ROM в RAM
* Обнуляет .bss


### Linker Script + Startup  

Компилятор (GCC или `riscv-as`) размещает секции так:
- `.text` — инструкции
- `.rodata` — константы (строки `"Hello"`)
- `.data` — инициализированные переменные
- `.bss` — нули (неинициализированные)
- Стек растёт вниз от высокого адреса RAM

При разделённой памяти **нужно явно делить адресное пространство** через linker script, потому что компилятор по умолчанию кладёт всё в одно пространство начиная с 0.

В объектном файле код и строка уже расположены в разных секциях.

Линкер располагает эти секции по адресам:

```
.text    -> 0x00000000     
.rodata  -> 0x10000000     
.data    -> 0x10001000    
```

Именно linker script сообщает линкеру:
* где должна располагаться .text;
* где должна располагаться .rodata;
* что .data во время выполнения находится в RAM (**VMA**);
* но ее начальный образ лежит в ROM (**LMA**).


так делают многие микроконтроллеры

```
Flash
   │
   └── .text
   └── .rodata

RAM
   └── .data
   └── .bss
```   

Но копировать нужно не все данные из ROM, а только изменяемые. 
* неизменяемые секции: для кода `.text` и `.rodata`, остаются в ROM. 
* секция `.data` хранит первонаяальное значение переменной которую мы можем менять, поэтому она копируется в RAM. 
* секция `.bss` используятся в RAM, просто обнуляем.


```
ROM
├── .text
├── .rodata
└── образ .data   (LMA)

        ↓ startup

RAM
├── .data (после копирования)
├── .bss
└── stack
```

**Linker Script файл linker.ld:**

```
/* linker.ld */
MEMORY
{
  rom (rx) : ORIGIN = 0x00000000, LENGTH = 2M
  ram (rwx): ORIGIN = 0x80000000, LENGTH = 8M
}

SECTIONS
{
  . = 0x00000000;

  .text :
  {
    *(.text*)
    *(.text.*)
  } > rom

  .rodata :
  {
    *(.rodata*)
    *(.rodata.*)
  } > rom

  __data_load = . ;      

  .data :
  {
    __data_start = . ;
    *(.data*)
    *(.sdata*)
    . = ALIGN(4);
    __data_end = . ;
  } > ram AT > rom

  .bss :
  {
    *(.bss*)
    *(.sbss*)
    . = ALIGN(4);
  } > ram
}
```

<br>

| Секция    | Где лежит после прошивки | Нужно копировать? | Почему                                                         |
| --------- | ------------------------ | ------------------| -------------------------------------------------------------- |
| `.text`   | ROM                      | Нет               | Код выполняется прямо из ROM.                                  |
| `.rodata` | ROM                      | Нет               | Константы и строки читаются прямо из ROM.                      |
| `.data`   | ROM (образ) → RAM        | Да                | Переменные должны иметь начальное значение и быть изменяемыми. |
| `.bss`    | RAM                      | Обнулить          | Неинициализированные переменные должны начинаться с нуля.      |

 

<br>
<details>
<summary> <b>Проверка работы процессора с секцией .rodata (чтение константы из ROM)</b></summary>


**Файл: test_rom.S:**

```asm
.section .rodata
value:
    .word 0x12345678

.section .text
.global _start

_start:
    la t0, value      # в t0 (он же x5) сохраняем адрес value
    lw t1, 0(t0)      # читаем из ROM в t1 он же x6  

loop:
    j loop
```

**Компиляция test_rom.S**

Создание файла ELF (Executable and Linkable Format) исполняемого типа, который содержит:
* ELF Header
  * содержит точку входа — адрес, с которого начинается выполнение программы.
  * содержит таблицу секций и таблицу сегментов — указатели на остальные части файла.
* таблицей сегментов Program Header Table для загрузчика.
* таблицей секций (Section Header Table), это информация для линковщика и отладчика, не используемая во время выполнения. Секции группируют данные по назначению:
  * `.text` — машинный код (инструкции)
  * `.data` — инициализированные глобальные переменные
  * `.rodata` — константы и строковые литералы
  * `.bss` — неинициализированные глобальные переменные (занимают место только в памяти, но не в файле)
  * `.symtab` — таблица символов (имена функций и переменных)
  * `.strtab` — строки с именами
  * `.debug_*` — отладочная информация (DWARF)
* сами данные


```
riscv64-unknown-elf-gcc -march=rv32i -mabi=ilp32 -nostdlib -T linker.ld test_rom.S -o test_rom.elf
```

| Флаг             | Расширения                   | Размер инструкций         |
|:---              |:---                          |:---                       |
| `-march=rv32i`   | Базовый 32-битный            | Только 32-битные          |
| `-march=rv32im`  | Базовый + умножение          | Только 32-битные          |
| `-march=rv32imc` | Базовый + умножение + сжатые | **16-битные** и 32-битные |
| `-march=rv64i`   | Базовый 64-битный            | Только 32-битные          |
| `-march=rv64im`  | Базовый + умножение          | Только 32-битные          |
| `-march=rv64imc` | Базовый + умножение + сжатые | **16-битные** и 32-битные |


Флаг `-march=rv32im` означает: используй 32-битные инструкции (I) и умножение (M), но без сжатых (C). Так как сжатые инструкции мы не реализовывали!

 
Флаг `-mabi=ilp32` задает правила вызова функций (ABI) для 32-битных RISC-V целей:
* i (int): Тип int — 32-битный
* lp (long, pointer): Тип long и указатели (void*) — также 32-битные
* 32: Указывает на общую 32-битную модель данных

Флаг `-mno-reraise-align` (по умолчанию включен) выборка инструкций только по выровненным адресам. (Выборка данных реализуется с поддержкой невыровненного доступа, так как возможен случай на границе слова, но для выборки инструкций мы этот функционал не реализовывали, адрес должен быть кратен двум для сжатых инструкций 16 бит, и кратен 4 для 32 битных инструкций)

> При сборке через ассемблер и линковщик:
> * riscv64-unknown-elf-as -march=rv32i test_rom.S -o test_rom.o
> * riscv64-unknown-elf-ld -Ttext=0x00000000 test_rom.o -o test_rom.elf



**Архитектура, тип ELF, разрядность**
```
file test_rom.elf
---

test_rom.elf: ELF 32-bit LSB executable, UCB RISC-V, soft-float ABI, version 1 (SYSV), statically linked, not stripped
```

**Размеры секций .text, .data, .bss и общий размер**
```
size test_rom.elf

---

   text    data     bss     dec     hex filename
     16       0       0      16      10 test_rom.elf
```     

**Показать заголовки секций**
```
riscv64-unknown-elf-objdump -h test_rom.elf

---

test_rom.elf:     file format elf32-littleriscv

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .text         0000000c  00000000  00000000  00001000  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  1 .rodata       00000004  0000000c  0000000c  0000100c  2**0
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  2 .riscv.attributes 0000001a  00000000  00000000  00001010  2**0
                  CONTENTS, READONLY
```

VMA (Virtual Memory Address) — это адрес, по которому процессор будет видеть секцию.

* `text    VMA = 0x00000000` значит первая инструкция будет по адресу `PC = 0x00000000` это полностью соответствует linker.ld: `rom (rx) : ORIGIN = 0x00000000`. Начало .text `VMA = 0x00000000` должно совпасть с Instruction Memory `0x00000000`
* `.rodata VMA = 0x0000000c` значит `.text` занимает `0xC` байт и после него сразу начинается `.rodata` (линкер просто кладет секции подряд). 
* Начало `.data` должно быть `0x80000000` пока его нет, потому что программа не содержит глобальных переменных.
* `LMA == VMA` потому что `.text -> rom` и `.rodata -> rom`. Если бы была .data, то увидел бы что-то вроде `VMA = 0x80000000` и `LMA = 0x00000029`

Это означает:
* во время выполнения данные находятся в RAM (0x80000000);
* но в ELF их начальный образ лежит сразу после `.rodata` в ROM.
* именно для этого мы писали `.data > ram AT > rom`


**Проверить адрес value**

Необходимо убедиться, что адрес данных по метке `value`, будет загружен в регистр `t0` (он же `x5`)

```
riscv64-unknown-elf-nm test_rom.elf

---

80000000 R __bss_end
80000000 R __bss_start
80000000 R __data_end
00000010 R __data_load
80000000 R __data_start
00000008 t loop
80800000 R __stack_top
00000000 T _start
0000000c r value          <--------
```

 
> Эта строка `0000000c r value`  означает, что `la t0, value` должно загрузить в `t0(x5)=0x0000000C`
>
> Т.е. мы уже знаем адрес `0x0000000C` по какому процессор будет читать ROM.


**Дизассемблировать секцию .text**

(Во что превратилась инструкция `la t0, value`)

```
riscv64-unknown-elf-objdump -d test_rom.elf

---

test_rom.elf:     file format elf32-littleriscv


Disassembly of section .text:

00000000 <_start>:
   0:   00c00293                li      t0,12
   4:   0002a303                lw      t1,0(t0)

00000008 <loop>:
   8:   0000006f                j       8 <loop>
```

>
> Т.е. ассемблерный код `la t0, value` превратился в `li t0, 12` т.е. в `addi t0, zero, 12`
>
> Если адрес влезает в 12 бит (диапазон -2048..2047), то в этом случае ассемблер/линковщик может оптимизировать `la` в одну инструкцию `addi` вместо двух (`lui+addi`)
>
> И размещает данные `value 0x12345678` таким образов в области адресов ROM, что бы одной инструкцией `lw t1, 0(t0)` их можно было склеить в `0x12345678`, поэтому адрес именно `0x0000000C` что бы попасть в нужный байт для инструкции `lw`
>
> компилятор так же не сжал инструкцию `j 8` в 16-ти битную форму `a001` т.е. в `c.j 0`, а выдал 32-х разрядную инструкцию `jal x0, 8` с кодом `0x0000006f`, так как мы явно задали флаг компиляции без сжатых инструкций `-march=rv32i`. 

**Дизассемблировать все секции с кодом**

```
riscv64-unknown-elf-objdump -D test_rom.elf

---

test_rom.elf:     file format elf32-littleriscv


Disassembly of section .text:

00000000 <_start>:
   0:   00c00293                li      t0,12
   4:   0002a303                lw      t1,0(t0)

00000008 <loop>:
   8:   0000006f                j       8 <loop>

Disassembly of section .rodata:

0000000c <value>:
   c:   5678                    .insn   2, 0x5678
   e:   1234                    .insn   2, 0x1234

...   
```

 
**Создание чистого бинарного файла (flat binary)**

Полный образ бинарника для прошивки, который загружается в память по конкретным адресам, т.е. objcopy упаковывает данные точно так, как они будут лежать в памяти (с учетом смещений)

```
riscv64-unknown-elf-objcopy -O binary test_rom.elf test_rom.bin
```


**дизассемблирование для RISC-V**
```
pip install tinyrv
tinyrv-dump test_rom.bin  

---

# addr  : instruction
00000000: addi       t0, zero, 12                  # rv_i
00000004: lw         t1, 0(t0)                     # rv_i
00000008: jal        zero, 0x8                     # rv_i
0000000c: c.lw       a4, 108(a2)                   # rv_c
0000000e: c.addi4spn a3, 0x128                     # rv_c
```

**Создание hex-dump**
```
(echo "v2.0 raw"; hexdump -v -e '1/4 "0x%08x\n"' test_rom.bin) > test_rom.hex
```

Либо, есть вариант онлайн, чтобы получить hex-дамп инструкций из файла ассемблера `program/test_rom.S`, можно вставить его содержимое в [Симулятор ассемблера Venus](https://venus.kvakil.me/) и выполнить Dump.


**Файл test_rom.hex:**
```
v2.0 raw
0x00c00293 # 0x0 # addi x5, x0, 12
0x0002a303 # 0x4 # lw x6, 0(x5)
0x0000006f # 0x8 # jal x0, 0
0x12345678 # 0xC # DATA
 
# Результат: t1 (x6) = 0x12345678
```
 

> Возможен вариант, когда компилятор разместит DATA вместе с 16-ти битной инструкцией (если не отключить ее при компиляции `-march=rv32i`):
> 
> ```
> 0x00a00293 # 0x0 # addi x5, x0, 10
> 0x0002a303 # 0x4 # lw x6, 0(x5)
> 0x5678a001 # 0x8 # DATA
> 0x00001234 # 0xC # DATA
> ```
> так как процессор будет читать ROM иснтрукцией `lw` по адресу `t0(x5)=0x0000000A`, то адрес начала как раз попадает в середину т.е. в третий байт инструкции-данных `0x5678a001` и склеивает ее с еще двумя младшими байтами инструкции-данных `0x00001234`, по итогу получаем готовое слово `0x12345678` которое сохраняется в `x6`
> 
> ```
> 00010010_00110100_                                    0x00001234
>                   01010110_01111000_1010000000000001  0x5678a001
> 00010010_00110100_01010110_01111000                   0x12345678
> ```                  
 

</details>

---

<br>
<details>
<summary> <b>Проверка работы процессора с секцией .rodata (чтение строкового литерала из ROM)</b></summary>


**Файл: test_rom_literal.S:**

```asm
.section .rodata
msg:
    .asciz "Hello"

.section .text
.global _start

_start:
    la t0, msg

loop:
    lbu t1, 0(t0)

    beqz t1, done

    li t2, 0x10000000
    sb t1, 0(t2)

    addi t0, t0, 1

    j loop

done:
    j done
```


**Компиляция test_rom.S**

  
```
riscv64-unknown-elf-gcc -march=rv32i -mabi=ilp32 -nostdlib -T linker.ld test_rom_literal.S -o test_rom_literal.elf

```

**Размеры секций .text, .data, .bss и общий размер**
```
size test_rom_literal.elf

---

   text    data     bss     dec     hex filename
     38       0       0      38      26 test_rom_literal.elf
```     


**Показать заголовки секций**
```
riscv64-unknown-elf-objdump -h test_rom_literal.elf

---

test_rom_literal.elf:     file format elf32-littleriscv

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .text         00000020  00000000  00000000  00001000  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  1 .rodata       00000006  00000020  00000020  00001020  2**0
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  2 .riscv.attributes 0000001a  00000000  00000000  00001026  2**0
                  CONTENTS, READONLY
```

VMA (Virtual Memory Address) — это адрес, по которому процессор будет видеть секцию.

* `text    VMA = 0x00000000` значит первая инструкция будет по адресу `PC = 0x00000000` это полностью соответствует linker.ld: `rom (rx) : ORIGIN = 0x00000000`. Начало .text `VMA = 0x00000000` должно совпасть с Instruction Memory `0x00000000`
* `.rodata VMA = 0x00000020` значит `.text` занимает `0x20` байт и после него сразу начинается `.rodata` (линкер просто кладет секции подряд). 
* Начало `.data` должно быть `0x80000000` пока его нет, потому что программа не содержит глобальных переменных.
* `LMA == VMA` потому что `.text -> rom` и `.rodata -> rom`. Если бы была .data, то увидел бы что-то вроде `VMA = 0x80000000` и `LMA = 0x00000029`

Это означает:
* во время выполнения данные находятся в RAM (0x80000000);
* но в ELF их начальный образ лежит сразу после `.rodata` в ROM.
* именно для этого мы писали `.data > ram AT > rom`

 
**Проверить адрес msg**

Необходимо убедиться что адрес данных по метке `msg`, будет загружен в регистр `t0` (он же `x5`) 

```
riscv64-unknown-elf-nm test_rom_literal.elf

---

80000000 R __bss_end
80000000 R __bss_start
80000000 R __data_end
00000026 R __data_load
80000000 R __data_start
0000001c t done
00000004 t loop
00000020 r msg            <------ 
80800000 R __stack_top
00000000 T _start
```

> Эта строка `00000020 r msg` означает, что `la t0, msg` должно загрузить в `t0(x5)=0x00000020`
>
> Т.е. мы уже знаем адрес `0x00000020` по какому процессор будет читать ROM.


**Дизассемблировать все секции с кодом**

```
riscv64-unknown-elf-objdump -D test_rom_literal.elf

---

test_rom_literal.elf:     file format elf32-littleriscv


Disassembly of section .text:

00000000 <_start>:
   0:   02000293                li      t0,32

00000004 <loop>:
   4:   0002c303                lbu     t1,0(t0)
   8:   00030a63                beqz    t1,1c <done>
   c:   100003b7                lui     t2,0x10000
  10:   00638023                sb      t1,0(t2) # 10000000 <__data_load+0xfffffda>
  14:   00128293                addi    t0,t0,1
  18:   fedff06f                j       4 <loop>

0000001c <done>:
  1c:   0000006f                j       1c <done>

Disassembly of section .rodata:

00000020 <msg>:
  20:   6548                    .insn   2, 0x6548
  22:   6c6c                    .insn   2, 0x6c6c
  24:   Address 0x24 is out of bounds.


...   
```
 
>
> Т.е. ассемблерный код `la t0, msg` превратился в `li t0, 32` т.е. в `addi t0, zero, 32`


**Создание чистого бинарного файла (flat binary)**

Полный образ бинарника для прошивки, который загружается в память по конкретным адресам, т.е. objcopy упаковывает данные точно так, как они будут лежать в памяти (с учетом смещений)

```
riscv64-unknown-elf-objcopy -O binary test_rom_literal.elf test_rom_literal.bin
```


**дизассемблирование для RISC-V**
```
pip install tinyrv
tinyrv-dump test_rom_literal.bin  

---

# addr  : instruction
00000000: addi       t0, zero, 32                  # rv_i
00000004: lbu        t1, 0(t0)                     # rv_i
00000008: beq        t1, zero, 0x1c                # rv_i
0000000c: lui        t2, 0x10000000                # rv_i
00000010: sb         t1, 0(t2)                     # rv_i
00000014: addi       t0, t0, 1                     # rv_i
00000018: jal        zero, 0x4                     # rv_i
0000001c: jal        zero, 0x1c                    # rv_i
00000020: c.ld       a0, 136(a0)                   # rv64_c, rv128_c
00000022: c.ld       a1, 216(fp)                   # rv64_c, rv128_c
```

**Создание hex-dump**
```
(echo "v2.0 raw"; hexdump -v -e '1/4 "0x%08x\n"' test_rom_literal.bin) > test_rom_literal.hex
```

Либо, чтобы получить hex-дамп инструкций из файла ассемблера `program/test_rom_literal.S`, можно вставить его содержимое в [Симулятор ассемблера Venus](https://venus.kvakil.me/) и выполнить Dump.

В ROM можно загрузить test_rom_literal.bin или test_rom_literal.hex.

**Файл test_rom_literal.hex:**

```
v2.0 raw
0x02000293 # 0x0  # addi x5, x0, 0x20 # загружае адрес начала строки
0x0002c303 # 0x4  # lbu x6, 0(x5)     # читаем данные из ROM[0x20] в x6
0x00030a63 # 0x8  # beq x6, x0, 20    # проверка не прочитали ли мы конец строки т.е. 0, иначе прыгнуть к PC[0x1C]=0x0000006f
0x100003b7 # 0xC  # lui x7, 65536     # x7 содержит адрес 0x10000 это терминал для вывода
0x00638023 # 0x10 # sb x6, 0(x7)      # запись в MMIO т.е. вывод в терминал символа содержащегося в x6
0x00128293 # 0x14 # addi x5, x5, 1    # инкремент на следующий символ
0xfedff06f # 0x18 # jal x0, -20       # прыгаем назад на 0x14 адресов т.е. на 5 инструкций к PC[0x4]=0x0002c303
0x0000006f # 0x1C # jal x0, 0         # PC = PC, процессор навсегда остается на этом адресе
0x6c6c6548 # 0x20 # DATA
0x0000006f # 0x24 # DATA

# Результат: откроется терминальное окно и напечатается Hello
```

Запись 0x6c6c6548 соответвует порядку байт в little-endian

```
Адрес   Байт  literal
---------------------
0x20    48    H
0x21    65    e
0x22    6C    l
0x23    6C    l
0x24    6F    o
0x25    00    \0
```
 


</details>

---


Дальнейшая проверка секций использует скрипт startup, он нужен только для `.data` и `.bss`
 

**Файл startup.S:**

```
.section .text
.global _start

.extern main

.extern __stack_top
.extern __data_load
.extern __data_start
.extern __data_end

.extern __bss_start
.extern __bss_end

_start:

    # 1. Инициализация стека
    la sp, __stack_top

    # 2. Копирование .data из ROM в RAM
    # цикл: ROM to RAM, копирует все слова секции .data
    la t0, __data_load
    la t1, __data_start
    la t2, __data_end

copy_data:
    beq t1, t2, copy_done

    lw t3, 0(t0)
    sw t3, 0(t1)

    addi t0, t0, 4
    addi t1, t1, 4

    j copy_data

copy_done:

    # 3. Обнулить .bss
    la t1, __bss_start
    la t2, __bss_end

clear_bss:
    beq t1, t2, bss_done
    sw zero, 0(t1)
    addi t1, t1, 4
    j clear_bss

bss_done:
    # 4. Вызвать main
    call main
    # 5. Если main вернулся — зависнуть

hang:
    j hang

```

Вместо просто безусловного перехода `j main` используем `call main`, Если оставить `j main`, то при возврате из main процессор попытается перейти по адресу из `ra`, который не был инициализирован. Компилятор и ABI предполагают, что main вызывается как обычная функция. Инструкция `call` записывает адрес возврата в регистр `ra`. Если когда-нибудь main выполнит `return 0;` то с `call` управление вернётся в startup, и мы сможем сделать `j hang` т.е. зациклить.



<br>
<details>
<summary><b>Проверка работы процессора с секцией .data (инициализация переменных с помощью скрипта startup)</b></summary>


**Файл startup.S для этого примера (без .bss):**
```
.section .text
.global _start

.extern main

.extern __stack_top
.extern __data_load
.extern __data_start
.extern __data_end

_start:

    # 1. Инициализация стека
    la sp, __stack_top

    # 2. Копирование .data из ROM в RAM
    la t0, __data_load
    la t1, __data_start
    la t2, __data_end

copy_data:
    beq t1, t2, copy_done

    lw t3, 0(t0)
    sw t3, 0(t1)

    addi t0, t0, 4
    addi t1, t1, 4

    j copy_data

copy_done:

    # 3. Обнулить .bss
    
    # 4. Вызвать main
    call main
    # 5. Если main вернулся — зависнуть

hang:
    j hang
```



**Файл test_data.c:**
```c
int counter = 5;

int main(void)
{
    while (counter > 4) {
        counter++;
    }
}
```

Компилятор ожидает, что перед вызовом main(): `RAM[0x80000000] = 5`, но после включения питания - RAM пустая! (`counter = мусор`)

Поэтому startup должен сделать (`counter = 5`):
```
ROM                   RAM

5  -----------------> 5
```


**Компиляция test_data.c**

```
riscv64-unknown-elf-gcc -march=rv32i -mabi=ilp32 -nostdlib -T linker.ld startup.S test_data.c -o test_data.elf
```    


**Архитектура, тип ELF, разрядность**
```
file test_data.elf

---

test_data.elf: ELF 32-bit LSB executable, UCB RISC-V, soft-float ABI, version 1 (SYSV), statically linked, not stripped
```

**Размеры секций .text, .data, .bss и общий размер**

```
size test_data.elf

---
   text    data     bss     dec     hex filename
    120       4       0     124      7c test_data.elf

```

В секции .data содержится 4 байта, это наш тип `sizeof(int)=4`

120 байт в секции .text говорит о 30 инструкциях (всех вместе и startup и main)


**Показать заголовки секций**

```
riscv64-unknown-elf-objdump -h test_data.elf

---
test_data.elf:     file format elf32-littleriscv

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .text         00000078  00000000  00000000  00001000  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  1 .data         00000004  80000000  00000078  00002000  2**2
                  CONTENTS, ALLOC, LOAD, DATA
  2 .bss          00000000  80000004  0000007c  00002004  2**0
                  ALLOC
  3 .riscv.attributes 0000001c  00000000  00000000  00002004  2**0
                  CONTENTS, READONLY
  4 .comment      00000022  00000000  00000000  00002020  2**0
                  CONTENTS, READONLY

```

LMA (Load Memory Address) это адрес, где эти данные лежат в образе прошивки. Для секции .data `LMA = 0x00000078` именно отсюда startup скопирует `counter` из `ROM[0x78]`. 

VMA (Virtual Memory Address) это адрес, по которому программа думает, что данные находятся во время выполнения. Для секции .data `VMA = 0x80000000` это значит переменная `counter` имеет адрес `0x80000000` и инструкция чтения обратится к `RAM[0x80000000]`.



Может появиться `.sdata`, а не `.data` - компилятор GCC для RISC-V, небольшие глобальные переменные, автоматически кладет в Small Data Section (`.sdata`)



**Проверить адреса меток**

```
riscv64-unknown-elf-nm test_data.elf

---

80000004 B __bss_end
80000004 B __bss_start
00000014 t copy_data     # локальная метка внутри startup.S. Она не экспортируется наружу, поэтому маленькая литера t.
0000002c t copy_done     # тоже локальная метка
80000000 D counter       # D = объект расположен в секции .data
80000004 D __data_end     
00000078 T __data_load
80000000 D __data_start  # текущее положение внутри секции .data, счётчик секции. Литера D означает область .data
00000030 T main          # глобальная функция
80800000 B __stack_top   # адрес, с которого стек должен начинать расти вниз: 0x80000000 + 8 MB = 0x80800000
00000000 T _start        # точка входа программы
```

Главное для startup — чтобы символы `__data_start` и `__data_end` охватывали все инициализированные данные.

Метка `__data_load` содержтит `0x00000078` адрес начала данных которые будут копироваться в RAM

startup копирует диапазон `[__data_start ; __data_end)`

```
RAM

0x80000000
│
├── counter (4 байта)
│
0x80000004
```


**Дизассемблировать все секции с кодом**

```
riscv64-unknown-elf-objdump -D test_data.elf

---

test_data.elf:     file format elf32-littleriscv


Disassembly of section .text:

00000000 <_start>:
   0:   07800293                li      t0,120
   4:   80000317                auipc   t1,0x80000
   8:   ffc30313                addi    t1,t1,-4 # 80000000 <counter>
   c:   80000397                auipc   t2,0x80000
  10:   ff838393                addi    t2,t2,-8 # 80000004 <__bss_end>

00000014 <copy_data>:
  14:   00730c63                beq     t1,t2,2c <copy_done>
  18:   0002ae03                lw      t3,0(t0)
  1c:   01c32023                sw      t3,0(t1)
  20:   00428293                addi    t0,t0,4
  24:   00430313                addi    t1,t1,4
  28:   fedff06f                j       14 <copy_data>

0000002c <copy_done>:
  2c:   0040006f                j       30 <main>

00000030 <main>:
  30:   ff010113                addi    sp,sp,-16
  34:   00812623                sw      s0,12(sp)
  38:   01010413                addi    s0,sp,16
  3c:   0180006f                j       54 <main+0x24>
  40:   800007b7                lui     a5,0x80000
  44:   0007a783                lw      a5,0(a5) # 80000000 <counter>
  48:   00178713                addi    a4,a5,1
  4c:   800007b7                lui     a5,0x80000
  50:   00e7a023                sw      a4,0(a5) # 80000000 <counter>
  54:   800007b7                lui     a5,0x80000
  58:   0007a703                lw      a4,0(a5) # 80000000 <counter>
  5c:   00400793                li      a5,4
  60:   fee7c0e3                blt     a5,a4,40 <main+0x10>
  64:   00000793                li      a5,0
  68:   00078513                mv      a0,a5
  6c:   00c12403                lw      s0,12(sp)
  70:   01010113                addi    sp,sp,16
  74:   00008067                ret

Disassembly of section .data:

80000000 <counter>:
80000000:       0005                    .insn   2, 0x0005
        ...

```


**Создание чистого бинарного файла (flat binary)**

```
riscv64-unknown-elf-objcopy -O binary test_data.elf test_data.bin
```

**дизассемблирование для RISC-V**

```
tinyrv-dump test_data.bin 

---

00000000: addi       t0, zero, 120                 # rv_i
00000004: auipc      t1, -0x80000000               # rv_i
00000008: addi       t1, t1, -4                    # rv_i
0000000c: auipc      t2, -0x80000000               # rv_i
00000010: addi       t2, t2, -8                    # rv_i
00000014: beq        t1, t2, 0x2c                  # rv_i
00000018: lw         t3, 0(t0)                     # rv_i
0000001c: sw         t3, 0(t1)                     # rv_i
00000020: addi       t0, t0, 4                     # rv_i
00000024: addi       t1, t1, 4                     # rv_i
00000028: jal        zero, 0x14                    # rv_i
0000002c: jal        zero, 0x30                    # rv_i
00000030: addi       sp, sp, -16                   # rv_i
00000034: sw         fp, 12(sp)                    # rv_i
00000038: addi       fp, sp, 16                    # rv_i
0000003c: jal        zero, 0x54                    # rv_i
00000040: lui        a5, 0x80000000                # rv_i
00000044: lw         a5, 0(a5)                     # rv_i
00000048: addi       a4, a5, 1                     # rv_i
0000004c: lui        a5, 0x80000000                # rv_i
00000050: sw         a4, 0(a5)                     # rv_i
00000054: lui        a5, 0x80000000                # rv_i
00000058: lw         a4, 0(a5)                     # rv_i
0000005c: addi       a5, zero, 4                   # rv_i
00000060: blt        a5, a4, 0x40                  # rv_i
00000064: addi       a5, zero, 0                   # rv_i
00000068: addi       a0, a5, 0                     # rv_i
0000006c: lw         fp, 12(sp)                    # rv_i
00000070: addi       sp, sp, 16                    # rv_i
00000074: jalr       zero, 0(ra)                   # rv_i
00000078: c.nop      nzimm6=1                      # rv_c
```

tinyrv-dump не знает, где заканчивается код, поэтому интерпретирует последние 4 байта `00000078` как инструкцию, но это DATA

**Создание hex-dump**

```
(echo "v2.0 raw"; hexdump -v -e '1/4 "0x%08x\n"' test_data.bin) > test_data.hex
```

В ROM можно загрузить test_data.bin или test_data.hex.

**Файл test_data.hex:**

```
v2.0 raw
                                            # Startup: копирование .data. 
0x07800293 # PC= 0x0   # addi x5, x0, 120   ## x5=120=0x78 это адрес __data_load в ROM по которому хранится DATA 0x00000005 
0x80000317 # PC= 0x4   # auipc x6, 0x80000  ## подготовка, x6 = 0x80000004 (imm + PC)
0xffc30313 # PC= 0x8   # addi x6, x6, -4    ## вычисляем __data_start, получаем x6 = 0x80000000                   
0x80000397 # PC= 0xC   # auipc x7, 0x80000  ## подготовка, x7 = 0x8000000C
0xff838393 # PC= 0x10  # addi x7, x7, -8    ## вычисляем __data_end, x7 = 0x80000004
0x00730c63 # PC= 0x14  # beq x6, x7, 24     ## если 0x80000000 != 0x80000004 то перепрыгнуть на 6 инструкций вперед к 0x0040006f (PC= 0x2C)
0x0002ae03 # PC= 0x18  # lw x28, 0(x5)      ## читаем ROM[0x78], в итоге x28=5
0x01c32023 # PC= 0x1C  # sw x28, 0(x6)      ## пишем в RAM[0x80000000]=5
0x00428293 # PC= 0x20  # addi x5, x5, 4     ## инкремент для след. итерации, двигаем адрес ROM для для след. слова DATA
0x00430313 # PC= 0x24  # addi x6, x6, 4     ## инкремент для след. итерации, двигаем адрес RAM __data_start для след. слова
0xfedff06f # PC= 0x28  # jal x0, -20        ## переход назад к 0x00730c63 (PC= 0x14)

0x0040006f # PC= 0x2C  # jal x0, 4         # Переход на main
0xff010113 # PC= 0x30  # addi x2, x2, -16  ## создаем стековый кадр # addi sp,sp,-16
0x00812623 # PC= 0x34  # sw x8, 12(x2)     ## сохраняем регистр     # sw s0,12(sp)
0x01010413 # PC= 0x38  # addi x8, x2, 16   ## настраиваем frame pointer
0x0180006f # PC= 0x3C  # jal x0, 0x18      ## формирования данных для условия, прижок на 0x800007b7  PC=0x3C(60)+0x18(24)=0x54(84). В objdum это выглядит так: j 54 <main+0x24>
                                           # Основной цикл
0x800007b7 # PC= 0x40 # lui x15, 0x80000   ## получаем адрес переменной counter x15=0x80000000
0x0007a783 # PC= 0x44 # lw x15, 0(x15)     ## читаем RAM[0x80000000] получаем текущее значение counter x15=5
0x00178713 # PC= 0x48 # addi x14, x15, 1   ## инкремент counter x14=6
0x800007b7 # PC= 0x4C # lui x15, 0x80000   ## снова получаем адрес переменной counter x15=0x80000000
0x00e7a023 # PC= 0x50 # sw x14, 0(x15)     ## пишем в RAM[0x80000000]=6
0x800007b7 # PC= 0x54 # lui x15, 0x80000   ## снова получаем адрес переменной counter x15=0x80000000
0x0007a703 # PC= 0x58 # lw x14, 0(x15)     ## читаем RAM[0x80000000] получаем x14=6
0x00400793 # PC= 0x5C # addi x15, x0, 4    ## формируем значение для сранения x15=4
0xfee7c0e3 # PC= 0x60 # blt x15, x14, -32  ## если 4 < 6 ? переходим назад на 8 инструкций к 0x800007b7 (PC= 0x40)

                                           # Если условие цикла станет ложным, выполняются последние инструкции
                                           # Это стандартный эпилог функции
0x00000793 # PC= 0x64 # addi x15, x0, 0    
0x00078513 # PC= 0x68 # addi x10, x15, 0   ## устанавливается возвращаемое значение (a0 = 0)
0x00c12403 # PC= 0x6C # lw x8, 12(x2)      ## восстанавливается сохранённый регистр s0
0x01010113 # PC= 0x70 # addi x2, x2, 16    ## освобождается место на стеке (sp += 16)
0x00008067 # PC= 0x74 # jalr x0, 0(x1)     ## выполняется возврат по адресу из ra
0x00000005 # PC= 0x78 # DATA

# Результат: значение counter будет аккумулироваться в x14=5...6...7...
```


 
Судя по основному циклу, компилятор GCC оптимизировал C код в это:
```c
goto check;

body:
    counter++;

check:
    if (counter > 4)
        goto body;

return 0;
```


А странная логика цикла (постонно читать адрес counter) обьясняется отсутвием флага оптимизации при компиляции. Поэтому компилятор:
* постоянно заново загружает адреса;
* часто перечитывает память;
* не сохраняет значения в регистрах дольше необходимого.

---

</details>

 
<br>
<details>
<summary><b>Проверка работы процессора с секцией .bss (обнуление неинициализированных глобальных переменных с помощью скрипта startup)</b></summary>

Следующим естественным шагом будет добавить в startup очистку секции .bss, чтобы заработали и неинициализированные глобальные переменные (int x; без присваивания).

 
**Файл test_bss.c:**
```c
int counter; // нет инициализатора, поэтому переменная попадет в .bss

int main(void)
{
    while (counter < 5)
    {
        counter++;
    }

    return 0;
}
```

После Reset содержимое RAM неизвестно, скрипт startup обнулит данные по используемым адресам.


**Компиляция test_bss.c**

```bash
riscv64-unknown-elf-gcc -march=rv32i -mabi=ilp32 -nostdlib -T linker.ld startup.S test_bss.c -o test_bss.elf
```    

**Архитектура, тип ELF, разрядность**
```bash
file test_bss.elf

---

test_bss.elf: ELF 32-bit LSB executable, UCB RISC-V, soft-float ABI, version 1 (SYSV), statically linked, not stripped
```

**Размеры секций .text, .data, .bss и общий размер**

```bash
size test_bss.elf

---

   text    data     bss     dec     hex filename
    164       0       4     168      a8 test_bss.elf
```

В секции .bss содержится 4 байта, это наш тип `sizeof(int)=4`

164 байта в секции .text говорит о 41 инструкции (всех вместе и startup и main)




**Показать заголовки секций**

```bash
riscv64-unknown-elf-objdump -h test_bss.elf

---
 
test_bss.elf:     file format elf32-littleriscv

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .text         000000a4  00000000  00000000  00001000  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  1 .data         00000000  80000000  000000a4  00002000  2**0
                  CONTENTS, ALLOC, LOAD, DATA
  2 .bss          00000004  80000000  000000a4  00002000  2**2
                  ALLOC
  3 .riscv.attributes 0000001c  00000000  00000000  00002000  2**0
                  CONTENTS, READONLY
  4 .comment      00000022  00000000  00000000  0000201c  2**0
                  CONTENTS, READONLY
```

Для .bss, ELF сообщает загрузчику: выдели 4 байта (00000004) памяти по адресу VMA (RAM[0x80000000]), но никаких данных из файла ROM туда копировать не нужно, так как стоит только флаг ALLOC (просто выделение памяти)


**Проверить адреса меток**

```bash
riscv64-unknown-elf-nm test_bss.elf

---

00000054 t bss_done
80000004 B __bss_end
80000000 B __bss_start  
00000044 t clear_bss
0000001c t copy_data
00000034 t copy_done
80000000 B counter       # B = объект расположен в секции .bss
80000000 D __data_end
000000a4 T __data_load
80000000 D __data_start
00000058 t hang
0000005c T main
80800000 B __stack_top
00000000 T _start
```

Цикл `clear_bss` записывает **0** в каждое слово диапазона `[__bss_start, __bss_end)`.
 
Именно этого требует стандарт языка C: все глобальные и статические переменные без явной инициализации должны быть равны нулю к моменту входа в `main()`.

**Дизассемблировать все секции с кодом**

```bash
riscv64-unknown-elf-objdump -D test_bss.elf

---
 
test_bss.elf:     file format elf32-littleriscv


Disassembly of section .text:

00000000 <_start>:
   0:   80800117                auipc   sp,0x80800
   4:   00010113                mv      sp,sp
   8:   0a400293                li      t0,164
   c:   80000317                auipc   t1,0x80000
  10:   ff430313                addi    t1,t1,-12 # 80000000 <counter>
  14:   80000397                auipc   t2,0x80000
  18:   fec38393                addi    t2,t2,-20 # 80000000 <counter>

0000001c <copy_data>:
  1c:   00730c63                beq     t1,t2,34 <copy_done>
  20:   0002ae03                lw      t3,0(t0)
  24:   01c32023                sw      t3,0(t1)
  28:   00428293                addi    t0,t0,4
  2c:   00430313                addi    t1,t1,4
  30:   fedff06f                j       1c <copy_data>

00000034 <copy_done>:
  34:   80000317                auipc   t1,0x80000
  38:   fcc30313                addi    t1,t1,-52 # 80000000 <counter>
  3c:   80000397                auipc   t2,0x80000
  40:   fc838393                addi    t2,t2,-56 # 80000004 <__bss_end>

00000044 <clear_bss>:
  44:   00730863                beq     t1,t2,54 <bss_done>
  48:   00032023                sw      zero,0(t1)
  4c:   00430313                addi    t1,t1,4
  50:   ff5ff06f                j       44 <clear_bss>

00000054 <bss_done>:
  54:   008000ef                jal     5c <main>

00000058 <hang>:
  58:   0000006f                j       58 <hang>

0000005c <main>:
  5c:   ff010113                addi    sp,sp,-16 # 807ffff0 <__bss_end+0x7fffec>
  60:   00812623                sw      s0,12(sp)
  64:   01010413                addi    s0,sp,16
  68:   0180006f                j       80 <main+0x24>
  6c:   800007b7                lui     a5,0x80000
  70:   0007a783                lw      a5,0(a5) # 80000000 <counter>
  74:   00178713                addi    a4,a5,1
  78:   800007b7                lui     a5,0x80000
  7c:   00e7a023                sw      a4,0(a5) # 80000000 <counter>
  80:   800007b7                lui     a5,0x80000
  84:   0007a703                lw      a4,0(a5) # 80000000 <counter>
  88:   00400793                li      a5,4
  8c:   fee7d0e3                bge     a5,a4,6c <main+0x10>
  90:   00000793                li      a5,0
  94:   00078513                mv      a0,a5
  98:   00c12403                lw      s0,12(sp)
  9c:   01010113                addi    sp,sp,16
  a0:   00008067                ret

```

**Создание чистого бинарного файла (flat binary)**

```bash
riscv64-unknown-elf-objcopy -O binary test_bss.elf test_bss.bin
```


**дизассемблирование для RISC-V**

```bash
tinyrv-dump test_bss.bin 

---

00000000: auipc      sp, -0x7f800000               # rv_i
00000004: addi       sp, sp, 0                     # rv_i
00000008: addi       t0, zero, 164                 # rv_i
0000000c: auipc      t1, -0x80000000               # rv_i
00000010: addi       t1, t1, -12                   # rv_i
00000014: auipc      t2, -0x80000000               # rv_i
00000018: addi       t2, t2, -20                   # rv_i
0000001c: beq        t1, t2, 0x34                  # rv_i
00000020: lw         t3, 0(t0)                     # rv_i
00000024: sw         t3, 0(t1)                     # rv_i
00000028: addi       t0, t0, 4                     # rv_i
0000002c: addi       t1, t1, 4                     # rv_i
00000030: jal        zero, 0x1c                    # rv_i
00000034: auipc      t1, -0x80000000               # rv_i
00000038: addi       t1, t1, -52                   # rv_i
0000003c: auipc      t2, -0x80000000               # rv_i
00000040: addi       t2, t2, -56                   # rv_i
00000044: beq        t1, t2, 0x54                  # rv_i
00000048: sw         zero, 0(t1)                   # rv_i
0000004c: addi       t1, t1, 4                     # rv_i
00000050: jal        zero, 0x44                    # rv_i
00000054: jal        ra, 0x5c                      # rv_i
00000058: jal        zero, 0x58                    # rv_i
0000005c: addi       sp, sp, -16                   # rv_i
00000060: sw         fp, 12(sp)                    # rv_i
00000064: addi       fp, sp, 16                    # rv_i
00000068: jal        zero, 0x80                    # rv_i
0000006c: lui        a5, 0x80000000                # rv_i
00000070: lw         a5, 0(a5)                     # rv_i
00000074: addi       a4, a5, 1                     # rv_i
00000078: lui        a5, 0x80000000                # rv_i
0000007c: sw         a4, 0(a5)                     # rv_i
00000080: lui        a5, 0x80000000                # rv_i
00000084: lw         a4, 0(a5)                     # rv_i
00000088: addi       a5, zero, 4                   # rv_i
0000008c: bge        a5, a4, 0x6c                  # rv_i
00000090: addi       a5, zero, 0                   # rv_i
00000094: addi       a0, a5, 0                     # rv_i
00000098: lw         fp, 12(sp)                    # rv_i
0000009c: addi       sp, sp, 16                    # rv_i
000000a0: jalr       zero, 0(ra)                   # rv_i
```


**Создание hex-dump**

```bash
(echo "v2.0 raw"; hexdump -v -e '1/4 "0x%08x\n"' test_bss.bin) > test_bss.hex
```

**Файл test_bss.hex:**


```text
v2.0 raw

                                             # ===== startup =====
0x80800117 # PC=0x00  auipc sp, 0x80800      # получить верхние 20 бит адреса стека
0x00010113 # PC=0x04  addi  sp, sp, 0        # sp = __stack_top = 0x80800000
0x0a400293 # PC=0x08  addi  x5, x0, 164      # x5(t0) = __data_load = адрес данных в ROM (0xA4)
0x80000317 # PC=0x0C  auipc x6, 0x80000      # сформировать адрес __data_start
0xff430313 # PC=0x10  addi  x6, x6, -12      # x6(t1) = __data_start = 0x80000000
0x80000397 # PC=0x14  auipc x7, 0x80000      # сформировать адрес __data_end
0xfec38393 # PC=0x18  addi  x7, x7, -20      # x7(t2) = __data_end = 0x80000000

                                             # ----- copy_data -----
0x00730c63 # PC=0x1C  beq   x6, x7, +24      # если .data пустая -> перейти к очистке .bss
0x0002ae03 # PC=0x20  lw    x28, 0(x5)       # прочитать слово из ROM
0x01c32023 # PC=0x24  sw    x28, 0(x6)       # записать слово в RAM
0x00428293 # PC=0x28  addi  x5, x5, 4        # следующий элемент ROM
0x00430313 # PC=0x2C  addi  x6, x6, 4        # следующий элемент RAM
0xfedff06f # PC=0x30  jal   x0, -20          # повторить copy_data

                                             # ===== clear_bss =====
0x80000317 # PC=0x34  auipc x6, 0x80000      # сформировать адрес __bss_start
0xfcc30313 # PC=0x38  addi  x6, x6, -52      # x6 = __bss_start = 0x80000000
0x80000397 # PC=0x3C  auipc x7, 0x80000      # сформировать адрес __bss_end
0xfc838393 # PC=0x40  addi  x7, x7, -56      # x7 = __bss_end = 0x80000004
0x00730863 # PC=0x44  beq   x6, x7, +16      # если вся .bss очищена -> вызвать main
0x00032023 # PC=0x48  sw    x0, 0(x6)        # RAM[x6] = 0
0x00430313 # PC=0x4C  addi  x6, x6, 4        # перейти к следующему слову
0xff5ff06f # PC=0x50  jal   x0, -12          # повторить clear_bss

                                             # ===== вызов main =====
0x008000ef # PC=0x54  jal   x1, +8           # вызвать main (ra = адрес возврата)

                                             # ===== если main когда-нибудь завершится =====
0x0000006f # PC=0x58  jal   x0, 0            # бесконечный цикл (hang)

                                             # ===== main() =====
0xff010113 # PC=0x5C  addi  sp, sp, -16      # создать стековый кадр
0x00812623 # PC=0x60  sw    x8, 12(sp)       # сохранить s0
0x01010413 # PC=0x64  addi  x8, sp, 16       # s0 = frame pointer
0x0180006f # PC=0x68  jal   x0, +24          # сразу перейти к проверке условия цикла

                                             # ----- тело цикла -----
0x800007b7 # PC=0x6C  lui   x15, 0x80000     # x15 = 0x80000000 (адрес counter)
0x0007a783 # PC=0x70  lw    x15, 0(x15)      # x15 = counter
0x00178713 # PC=0x74  addi  x14, x15, 1      # x14 = counter + 1
0x800007b7 # PC=0x78  lui   x15, 0x80000     # снова адрес counter
0x00e7a023 # PC=0x7C  sw    x14, 0(x15)      # counter = counter + 1

                                             # ----- проверка условия -----
0x800007b7 # PC=0x80  lui   x15, 0x80000     # адрес counter
0x0007a703 # PC=0x84  lw    x14, 0(x15)      # x14 = counter
0x00400793 # PC=0x88  addi  x15, x0, 4       # x15 = 4
0xfee7d0e3 # PC=0x8C  bge   x15, x14, -32    # если (4 >= counter) перейти к телу цикла

                                             # ----- return 0 -----
0x00000793 # PC=0x90  addi  x15, x0, 0       # x15 = 0
0x00078513 # PC=0x94  addi  x10, x15, 0      # a0 = 0 (возвращаемое значение)
0x00c12403 # PC=0x98  lw    x8, 12(sp)       # восстановить s0
0x01010113 # PC=0x9C  addi  sp, sp, 16       # удалить стековый кадр
0x00008067 # PC=0xA0  jalr  x0, 0(x1)        # return

# Результат: регистр x14=5 и процессор в вечном цикле на инструкции 0x0000006f (PC=0x54)
```

В ROM можно загрузить test_bss.bin или test_bss.hex. 


</details>

---

Реальные процессоры обычно используют **модифицированную гарвардскую архитектуру**. Исходно RAM одна, как в архитектуре Фон Неймана, но она разделена на две независимые памяти I-Cache для инструкций и D-Cache для данных, в процессе работы эти кеши загружаются (что требует некоторого простоя) и процессор использует уже независимые шины как чистой гарвардской архитектуре. Т.е. у ядра есть две независимые шины, в один такт процессор может получить следующую инструкцию и прочитать/записать данные. 

```
          CPU
       /       \
   I-Cache    D-Cache
   (Program)  (Data)
        \      /
        L2 Cache
           |
        L3 Cache
           |
  RAM (Program + Data)
```


---

## Регистры

### Общие регистры (GPR)

| Архитектурное имя | ABI имя | Dec | Hex  | Binary |
|---------|---------|-----|----- |--------|
| x0      | zero    | 0   | 0x00 | 00000  |
| x1      | ra      | 1   | 0x01 | 00001  |
| x2      | sp      | 2   | 0x02 | 00010  |
| x3      | gp      | 3   | 0x03 | 00011  |
| x4      | tp      | 4   | 0x04 | 00100  |
| x5      | t0      | 5   | 0x05 | 00101  |
| x6      | t1      | 6   | 0x06 | 00110  |
| x7      | t2      | 7   | 0x07 | 00111  |
| x8      | s0/fp   | 8   | 0x08 | 01000  |
| x9      | s1      | 9   | 0x09 | 01001  |
| x10     | a0      | 10  | 0x0A | 01010  |
| x11     | a1      | 11  | 0x0B | 01011  |
| x12     | a2      | 12  | 0x0C | 01100  |
| x13     | a3      | 13  | 0x0D | 01101  |
| x14     | a4      | 14  | 0x0E | 01110  |
| x15     | a5      | 15  | 0x0F | 01111  |
| x16     | a6      | 16  | 0x10 | 10000  |
| x17     | a7      | 17  | 0x11 | 10001  |
| x18     | s2      | 18  | 0x12 | 10010  |
| x19     | s3      | 19  | 0x13 | 10011  |
| x20     | s4      | 20  | 0x14 | 10100  |
| x21     | s5      | 21  | 0x15 | 10101  |
| x22     | s6      | 22  | 0x16 | 10110  |
| x23     | s7      | 23  | 0x17 | 10111  |
| x24     | s8      | 24  | 0x18 | 11000  |
| x25     | s9      | 25  | 0x19 | 11001  |
| x26     | s10     | 26  | 0x1A | 11010  |
| x27     | s11     | 27  | 0x1B | 11011  |
| x28     | t3      | 28  | 0x1C | 11100  |
| x29     | t4      | 29  | 0x1D | 11101  |
| x30     | t5      | 30  | 0x1E | 11110  |
| x31     | t6      | 31  | 0x1F | 11111  |

В RISC-V регистр `x0` всегда равен 0. Любая попытка записи в него должна игнорироваться.

Эти имена соответствуют использованию регистров в стандартном двоичном интерфейсе прикладных программ (application binary interface, ABI) RISC-V:
* `рс` — содержит 32-разрядный программный счетчик, содержащий адрес теку­щей инструкции;
* `ra` — адрес возврата функции;
* `sp` — указатель стека;
* `gp` — глобальный указатель данных;
* `tp` — локальный указатель данных (на уровне потока);
* `t0—t6` — временное хранение;
* `fр` — указатель кадра для данных локального стека (на уровне функции) (этот вариант использования не является обязательным);
* `s0-s11`— сохраняемые регистры (если указатель кадра не используется, x8 становится s0);
* `а0—а7` — аргументы, передаваемые в функции. Любые дополнительные аргу­менты передаются в стек. Возвращаемые функцией значения передаются в `а0` и `а1`.

Имена `x0–x31` — это архитектурные имена 32 общих регистров RISC-V. Процессор знает только эти номера регистров.

Имена `zero`, `ra`, `sp`, `gp`, `tp`, `a0`, `s0`, `t0` и другие — это ABI-имена (Application Binary Interface). Являются лишь удобными псевдонимами для `x0–x31`.

ABI определяет соглашение о том, **как программы должны использовать регистры**, чтобы объектные файлы, библиотеки, компилятор, операционная система и отладчик могли корректно взаимодействовать друг с другом.
 
Если писать программу полностью самостоятельно на ассемблере и не вызывать чужие функции, то можно использовать любой регистр практически для любых целей.

Но если программа взаимодействует с кодом, скомпилированным GCC, использует стандартные библиотеки или работает под операционной системой, необходимо соблюдать ABI. Например, если использовать `ra` (`x1`) как обычный регистр и не сохранять в нём адрес возврата, то инструкция `ret` не сможет вернуться из функции, и выполнение программы нарушится.

Поэтому ABI нужно соблюдать не потому, что этого требует процессор, а потому, что этого требуют соглашения между всем программным стеком (компилятор, библиотеки, вызываемый код и ОС при необходимости) который рассчитывает на соблюдение ABI.


В RISC-V отсутствует инструкция, которая просто перемещает содержимое одного регистра в другой. Вместо этого инструкция сложения в RISC-V складывает значение из регистра-источника и непосредственное значение, равное нулю, и со­храняет результат в регистре-приемнике, получая тот же результат. Таким образом, инструкция для перемещения содержимого регистра `х2` в регистр `х1` выглядит так: `add x1,x2,0` — она помещает значение (`х2 + 0`) в регистр `х1`.

Почти во всех форматах инструкции позиции регистров одинаковые:

| Поле  | Биты   | Размер |
| ----- | ------ |------  |
| `rd`  | 11..7  | 5 бит  |
| `rs1` | 19..15 | 5 бит  |
| `rs2` | 24..20 | 5 бит  |

### Специальные регистры процессора

Это регистры, которые существуют внутри процессора (нельзя обратиться напрямую из инструкции): PC, mepc, mcause, mtvec, mstatus, cycle

### Memory-mapped register

Выбранные адреса для периферийных регистров **данной** реализации процессора.

Периферийные регистры в адресном в пространстве MMIO:
* addr 0x10000004  	mtimecmp_lo
* addr 0x10000008   mtimecmp_hi
* addr 0x10000010   ExternalInterruptReset
* addr 0x10000018   SoftwareInterruptReset
* addr 0x1000000C   Display CTRL (Регистр B)
  
## Типы инструкций

Базовый набор инструкций RISC-V состоит всего из 47 инструкций. Восемь из них — это системные инструкции, которые выполняют системные вызовы и получают доступ к счетчикам производительности. Остальные 39 инструкций относятся к категориям вычислительных инструкций, инструкций потока управления и инст­рукций доступа к памяти.
 
Однотактный (Single Cycle) процессор архитектуры  RV32I включает в себя фиксированный набор из 40 уникальных инструкций.

Как по 7-ми битному полю операции (opcode — это биты Instruction[6:0]) аппаратно определяется тип кодирования инструкции.
 
В архитектуре RISC-V младшие два бита `opcode[1:0]` для всех стандартных 32-х битных инструкций всегда равны `11`. Соответственно, тип инструкции и её назначение полностью определяются старшими пятью битами `opcode[6:2]`.

 

| Тип инструкции         | Opcode (двоичный) | Opcode (hex) | Примеры команд                                                | Описание назначения                     |
| ---------------------- | ----------------- | ------------ | ------------------------------------------------------------- | ----------------------------------      |
| R-type                 | `0110011`         | `0x33`       | `add`, `sub`, `and`, `or`, `xor`, `sll`, `srl`, `sra`, `slt`  | Арифметика/логика между регистрами      |
| I-type (ALU immediate) | `0010011`         | `0x13`       | `addi`, `andi`, `ori`, `xori`, `slli`, `srli`, `srai`, `slti` | Операции с immediate                    |
| I-type (LOAD)          | `0000011`         | `0x03`       | `lb`, `lh`, `lw`, `lbu`, `lhu`                                | Загрузка из памяти                      |
| I-type (JALR)          | `1100111`         | `0x67`       | `jalr`                                                        | Косвенный переход                       |
| I-type (System)        | `1110011`         | `0x73`       | `ecall`, `ebreak`, CSR-инструкции                             | Системные вызовы, отладка, работа с CSR |
| S-type                 | `0100011`         | `0x23`       | `sb`, `sh`, `sw`                                              | Запись в память                         |
| B-type                 | `1100011`         | `0x63`       | `beq`, `bne`, `blt`, `bge`, `bltu`, `bgeu`                    | Условные переходы                       |
| U-type (LUI)           | `0110111`         | `0x37`       | `lui`                                                         | Загрузка верхних 20 бит                 |
| U-type (AUIPC)         | `0010111`         | `0x17`       | `auipc`                                                       | PC-relative арифметика                  |
| J-type                 | `1101111`         | `0x6F`       | `jal`                                                         | Безусловный переход                     |



<br>
<details>
<summary> <b> 1. Арифметико-логические операции (ALU) </b> </summary>
 
Эти инструкции выполняют вычисления над регистрами и непосредственными значениями (константами).

Регистр-регистр (R-тип):
*   `add`, `sub` – сложение и вычитание 
*   `and`, `or`, `xor` – побитовые И, ИЛИ, исключающее ИЛИ 
*   `sll`, `srl`, `sra` – логический/арифметический сдвиг влево/вправо 
*   `slt`, `sltu` – установка флага, если меньше (со знаком/без знака) 

Регистр-немедленное значение (I-тип):
*   `addi` – сложение с константой 
*   `andi`, `ori`, `xori` – побитовые операции с константой 
*   `slli`, `srli`, `srai` – сдвиг на константу 
*   `slti`, `sltiu` – сравнение с константой 

</details>

<br>
<details>
<summary> <b> 2. Загрузка/Сохранение в память (Load/Store) </b> </summary>
 
Доступ к данным в памяти возможен только через эти инструкции.

Загрузка (I-тип):
*   `lb`, `lbu` – загрузка байта (со знаком/без знака) 
*   `lh`, `lhu` – загрузка полуслова (16 бит) 
*   `lw` – загрузка слова (32 бита) 

Сохранение (S-тип):
*   `sb` – сохранение байта 
*   `sh` – сохранение полуслова 
*   `sw` – сохранение слова 

</details>

<br>
<details>
<summary> <b> 3. Управление потоком (Branch & Jump) </b> </summary>
 
Обеспечивают изменение последовательности выполнения кода.

Условные переходы (B-тип):
*   `beq`, `bne` – переход, если равно / не равно 
*   `blt`, `bge` – переход, если меньше / больше или равно (со знаком) 
*   `bltu`, `bgeu` – переход, если меньше / больше или равно (без знака) 

Безусловные переходы:
*   `jal` (J-тип) – безусловный переход с сохранением адреса возврата 
*   `jalr` (I-тип) – безусловный переход по адресу из регистра 

</details>

<br>
<details>
<summary> <b> 4. Загрузка констант и вспомогательные </b> </summary>
 
*   `lui` (U-тип) – загрузка старших 20 бит константы в регистр 
*   `auipc` (U-тип) – прибавление константы к текущему значению PC 

</details>

<br>
<details>
<summary> <b> 5. Служебные инструкции (опционально) </b> </summary>
 
*   `ecall` – вызов системной среды (исключение) 
*   `ebreak` – вызов отладчика (точка останова) 
*   `fence` – упорядочивание операций с памятью (в простых однотактных ядрах часто реализуется как "нет операции" - NOP) 

</details>

---

## Инструкции I-типа

Immediate arithmetic (I-type, OP-IMM)
 

Opcode:
* I-type (ALU immediate)  `0010011` 
* I-type (LOAD)           `0000011`
* I-type (JALR)           `1100111`
* I-type (System)         `1110011`     

| imm    | rs1 (регистр-источник)| funct3 (подфункция) | rd (регистр-приёмник) | opcode (код операции) |
|---     |---                    |---                  |---                    |---                    |
| 31:20  | 19:15                 | 14:12               | 11:7                  | 6:0                   |
| 12 бит | 5 бит                 | 3 бита              | 5 бит                 | 7 бит                 |

`rd` - Адрес регистра-приёмника. Сюда процессор запишет результат вычисления. (Всего регистров 32, поэтому 5 бит — это ровно 32 варианта, от `x0` до `x31`)

`funct3` - Это 3-битный «переключатель». Он уточняет, какую именно операцию сделать. Например, для `addi` это `000`, а для `slli` — `001`.

`rs1` - Адрес регистра-источника. Отсюда процессор читает первое (и часто единственное) число для вычисления.

Для I-type immediate в RV32I используется 12-битное знаковое число (signed).
В `imm[11]` находится знак числа, а в остальной части `imm[10:0]` константа.

Максимальное положительное imm:  2047 (011111111111, 0x7FF)

Максимальное отрицательное imm: -2048 (100000000000, 0x800)


Поле константы представляет собой 12-битное число со знаком (в дополнительном коде) для всех инструкций типа I, кроме инструкций непосредственного сдвига `slli`, `srli` и `srai`.
Для этих трех инструкций сдвига поле $imm_{4:0}$ представляет собой 5-битное значение сдвига без знака; верхние семь бит $imm$ равны 0 для инструкций `srli` и `slli`, но инструкция `srai` помещает 1 в $imm_{10}$ (т. е. 30-й бит инструкции).

В поле imm у нас только 12 бит, но регистры в RISC-V 32-битные. Как же записать 12-битное число в 32-битный регистр? Старший бит константы (бит №31 в инструкции) — это знак (плюс или минус). При выполнении процессор дублирует этот бит во все оставшиеся 20 старших бит регистра:
* Если `imm[11] = 0` (число положительное): регистр получит `0x00000XXX`.
* Если `imm[11] = 1` (число отрицательное): регистр получит `0xFFFFFXXX` (все старшие биты станут единицами).

Это позволяет работать с числами в диапазоне от -2048 до +2047 прямо внутри инструкции.

### Группа: Арифметика 

(opcode = 0010011)

Здесь всегда выполняется: `rd = rs1 (операция) imm`

| Команда     | funct3                  | funct7  | Что делает                                                       | Пример                             |
| :---        | :---                    | :---    | :---                                                             | :---                               |
| **`addi`**  | `000`                   | -       | Сложение регистра и константы: `rd = rs1 + imm`                  | `addi x5, x6, 10` → `x5 = x6 + 10` |
| **`slti`**  | `010`                   | -       | **S**et **L**ess **T**han **I**mmediate: `rd = (rs1 < imm)`      | `slti x5, x6, 5`                   |
| **`sltiu`** | `011`                   | -       | То же самое, но сравнение **БЕЗ** знака (как беззнаковые числа). | `sltiu x5, x6, 5`                  |
| **`andi`**  | `111`                   | -       | Побитовое **И** (AND) с константой: `rd = rs1 & imm`             | `andi x5, x6, 15`                  |
| **`ori`**   | `110`                   | -       | Побитовое **ИЛИ** (OR) с константой: `rd = rs1 | imm`            | `ori x5, x6, 3`                    |
| **`xori`**  | `100`                   | -       | Побитовое исключающее **ИЛИ** (XOR): `rd = rs1 ^ imm`            | `xori x5, x6, -1` (инверсия)       |
| **`slli`**  | `001`                   | 0000000 | Логический сдвиг **влево** на константу (от 0 до 31).            | `slli x5, x6, 2` (умножить на 4)   |
| **`srli`**  | `101`                   | 0000000 | Логический сдвиг **вправо** (заполняет нулями).                  | `srli x5, x6, 1` (поделить на 2)   |
| **`srai`**  | `101` (но особый случай)| 0100000 | Арифметический сдвиг **вправо** (сохраняет знак).                | `srai x5, x6, 1`                   |

<br>
<details>
<summary> <b> # 1 Instruction: addi </b> </summary>

Инструкция `addi` - сложение регистра-источника `rs1` и константы `imm` с сохранением результата в регистре-приёмнике `rd` 

Пример:

```
addi rd, rs1, imm
---
addi x10, x0, -2
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа)  |
| ---    | ---                   | ---                    | ---              |
| addi   | x10                   | x0                     | -2               |


```
hexadecimal  : 0xffe00513
Assembly     : addi x10, x0, 0xFFE
Format       : I (ALU immediate)
instr        : 11111111111000000000010100010011
opcode       : .........................0010011 (0x13)
rd (receiver): ....................01010....... x10 ("a0")
funct3       : .................000............ (addi)
rs1 (source) : ............00000............... x0 ("zero")
imm          : 111111111110.................... raw=0xFFE signed=-2 decimal
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0xffe00513   # addi x10, x0, -2
> ```
> Результат в x10=0xFFE

---

</details>

<br>
<details>
<summary> <b> # 2 Instruction: slti </b> </summary>

Инструкция `slti` (**S**et **L**ess **T**han **I**mmediate) сравнение на «меньше» со знаком: `rd = (rs1 < imm) ? 1 : 0`

Выполняет сравнение со знаком, если значение в регистре-источнике `rs1` меньше константы `imm` то в целевой регистр-приёмник `rd` запишется 1, иначе 0.
 
```
slti rd, rs1, imm
---
slti x4, x10, -1
```

```
x4 = x10 < -1? 1:0;
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа) |
| ---    | ---                   | ---                    | ---             |
| slti   | x4                    | x10                    | -1              |


```
hexadecimal  : 0xFFF52213
Assembly     : slti x4, x10, -1
FORMAT       : I (ALU immediate)
instr        : 11111111111101010010001000010011
opcode       : .........................0010011 (0x13)
rd (receiver): ....................00100....... x4 ("tp")
funct3       : .................010............ (slti)
rs1 (source) : ............01010............... x10 ("a0")
imm          : 111111111111.................... (4095 decimal, 0xFFFFFFFF hex)
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0xffe00513   # addi x10, x0, -2
> 0xfff52213   # slti x4, x10, -1
> ```
> Результат в x4=1

---

</details>

<br>
<details>
<summary> <b> # 3 Instruction: sltiu </b> </summary>

Инструкция `sltiu` (**S**et **L**ess **T**han **I**mmediate **U**nsigned) сравнение на «меньше» без знака: `rd = (rs1 < imm) ? 1 : 0`

Выполняет сравнение без знака, если значение в регистре-источнике `rs1` меньше константы `imm` то в целевой регистр-приёмник `rd` запишется 1, иначе 0.


```
sltiu rd, rs1, imm
---
sltiu x4, x10, 3
```

```
x4 = x10 < 3? 1:0;
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа) |
| ---    | ---                   | ---                    | ---             |
| sltiu  | x4                    | x10                    | 3               |


```
hexadecimal  : 0x00353213
Assembly     : sltiu x4, x10, 3
FORMAT       : I (ALU immediate)
instr        : 00000000001101010011001000010011
opcode       : .........................0010011 (0x13)
rd (receiver): ....................00100....... x4 ("tp")
funct3       : .................011............ (sltiu)
rs1 (source) : ............01010............... x10 ("a0")
imm          : 000000000011.................... (3 decimal, 0x003 hex)
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0x00200513   # addi x10, x0, 2
> 0x00353213   # sltiu x4, x10, 3
> ```
> Результат в x4=1

---

</details>

<br>
<details>
<summary> <b> # 4 Instruction: andi </b> </summary>

Инструкция `andi` - операция побитовое AND между содержимым регистра-источника `rs1` и непосредственным значением константы `imm`, с сохранением результата в регистре-приёмнике `rd` 

```
andi rd, rs1, imm
---
andi x1, x10, 3
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа)  |
| ---    | ---                   | ---                    | ---              |
| andi   | x1                    | x10                    | 3                |

```
hexadecimal  : 0x00357093
Assembly     : andi x1, x10, 0x3
Format       : I (ALU immediate)
instr        : 00000000001101010111000010010011
opcode       : .........................0010011 (0x13)
rd (receiver): ....................00001....... x1 ("ra")
funct3       : .................111............ (andi)
rs1 (source) : ............01010............... x10 ("a0")
imm          : 000000000011.................... raw=0x3 signed=3 decimal
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0x00200513   # addi x10, x0, 2
> 0x00357093   # andi x1, x10, 3
> ```
> Результат в x1=2

---

</details>

<br>
<details>
<summary> <b> # 5 Instruction: ori </b> </summary>

Инструкция `ori` - операция побитовое OR между содержимым регистра-источника `rs1` и непосредственным значением `imm`, результат которой записывается в регистр-приёмник `rd`

```
ori rd, rs1, imm
---
ori x1, x10, 3
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа)  |
| ---    | ---                   | ---                    | ---              |
| ori    | x1                    | x10                    | 3                |


```
hexadecimal  : 0x00356093
Assembly     : ori x1, x10, 0x3
Format       : I (ALU immediate)
instr        : 00000000001101010110000010010011
opcode       : .........................0010011 (0x13)
rd (receiver): ....................00001....... x1 ("ra")
funct3       : .................110............ (ori)
rs1 (source) : ............01010............... x10 ("a0")
imm          : 000000000011.................... raw=0x3 signed=3 decimal
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0x00200513   # addi x10, x0, 2
> 0x00356093   # ori x1, x10, 3
> ```
> Результат в x1=3

---

</details>

<br>
<details>
<summary> <b> # 6 Instruction: xori </b> </summary>

Инструкция `xori` - операция побитового XOR между регистром `rs1` и константой `imm`, результат пишется в `rd`

```
xori rd, rs1, imm
---
xori x1, x10, 3
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа) |
| ---    | ---                   | ---                    | ---             |
| xori   | x1                    | x10                    | 3               |


```
hexadecimal  : 0x00354093
Assembly     : xori x1, x10, 0x3
Format       : I (ALU immediate)
instr        : 00000000001101010100000010010011
opcode       : .........................0010011 (0x13)
rd (receiver): ....................00001....... x1 ("ra")
funct3       : .................100............ (xori)
rs1 (source) : ............01010............... x10 ("a0")
imm          : 000000000011.................... raw=0x3 signed=3 decimal
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0x00200513   # addi x10, x0, 2
> 0x00354093   # xori x1, x10, 3
> ```
> Результат в x1=1

---

</details>

Обратите внимание на **`slli`**, **`srli`** и **`srai`**. 
В этих командах 12-битное поле `imm` используется хитро. Младшие 5 бит `imm` (биты 20–24) — это **количество сдвига** (от 0 до 31). А биты 25–31 должны быть строго равны нулю (иначе инструкция считается недействительной). 
Для сдвигов **не используется** знаковое расширение!

| funct7 | imm (shamt) | rs1 (регистр-источник)| funct3 (подфункция) | rd (регистр приёмник) | opcode (код операции) |
|---     |---          |---                    |---                  |---                    |---                    |
| 31:25  | 24:20       | 19:15                 | 14:12               | 11:7                  | 6:0                   |
| 7 бит  | 5 бит       | 5 бит                 | 3 бита              | 5 бит                 | 7 бит                 |
  
где:
* shamt — величина сдвига (5 бит для RV32)
* funct7=0000000 → SLLI, SRLI
* funct7=0100000 → SRAI

> Отличие между логическим и арифметическим сдвигом в том, чем заполняется освободившееся место после сдвига:
> * Логический сдвиг - заполняется нулями
> * Арифметический сдвиг - заполняется битом знака (старший бит)

<br>
<details>
<summary> <b> # 7 Instruction: slli </b> </summary>

Инструкция `slli` (**S**hift **L**eft **L**ogical **I**mmediate) выполняет логический сдвиг влево значения в `rs1` на заданную константу в `imm`, результат пишется в `rd`
 
Величина imm содержит 5 бит shamt (shift amount) — количество бит сдвига, диапазон 0 - 31

```
slli rd, rs1, shamt
---
slli x1, x10, 1
```

```
rs1 << shamt = rd
x10 << shamt = x1
2 << 1       = 4
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (shamt) |
| ---    | ---                   | ---                    | ---         |
| 7 бит  | 5 бит                 | 5 бит                  | 5 бит       |
| slli   | x1                    | x10                    | 1           |


```
hexadecimal  : 0x00151093
Assembly     : slli x1, x10, 0x1
Format       : I (ALU immediate)
instr        : 00000000000101010001000010010011
opcode       : .........................0010011 (0x13)
rd (receiver): ....................00001....... x1 ("ra")
funct3       : .................001............ (slli)
rs1 (source) : ............01010............... x10 ("a0")
imm shamt    : .......00001.................... (1)
funct7       : 0000000......................... (0)
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0x00200513   # addi x10, x0, 2
> 0x00151093   # slli x1, x10, 1
> ```
> Результат в x1=4

</details>

<br>
<details>
<summary> <b> # 8 Instruction: srli </b> </summary>

Инструкция `srli` (**S**hift **R**ight **L**ogical **I**mmediate) выполняет логический сдвиг вправо значения в `rs1` на заданную константу в `imm`, результат пишется в `rd`

```
srli rd, rs1, shamt
---
srli x1, x10, 1
```

```
rs1 >> shamt = rd
x10 >> shamt = x1
6  >> 1      = 3
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (shamt) |
| ---    | ---                   | ---                    | ---         |
| 7 бит  | 5 бит                 | 5 бит                  | 5 бит       |
| srli   | x1                    | x10                    | 1           |


```
hexadecimal  : 0x00155093
Assembly     : srli x1, x10, 0x1
Format       : I (ALU immediate)
instr        : 00000000000101010101000010010011
opcode       : .........................0010011 (0x13)
rd (receiver): ....................00001....... x1 ("ra")
funct3       : .................101............ (srli)
rs1 (source) : ............01010............... x10 ("a0")
imm shamt    : .......00001.................... (1)
funct7       : 0000000......................... (0)
```


> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0x00600513   # addi x10, x0, 6
> 0x00155093   # srli x1, x10, 1
> ```
> Результат в x1=3

</details>

<br>
<details>
<summary> <b> # 9 Instruction: srai </b> </summary>

Инструкция `srai` (**S**hift **R**ight **A**rithmetic **I**mmediate) выполняет арифметический сдвиг вправо значения в `rs1` на заданную константу в `imm`, результат пишется в `rd`

> старшие биты (слева) заполняет битом знака (MSB)

```
srai rd, rs1, shamt
---
srai x1, x10, 1
```

```
11111000 >> 1   = 11111100 
0xFF8    >> 1   = 0xFFC     
-8       >> 1   = -4        
x10      >> imm = x1        
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (shamt) |
| ---    | ---                   | ---                    | ---         |
| 7 бит  | 5 бит                 | 5 бит                  | 5 бит       |
| srai   | x1                    | x10                    | 1           |


```
hexadecimal  : 0x40155093
Assembly     : srai x1, x10, 0x1
Format       : I (ALU immediate)
instr        : 01000000000101010101000010010011
opcode       : .........................0010011 (0x13)
rd (receiver): ....................00001....... x1 ("ra")
funct3       : .................101............ (srai)
rs1 (source) : ............01010............... x10 ("a0")
imm shamt    : .......00001.................... (1)
funct7       : 0100000......................... (32)
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0xff800513   # addi x10, x0, -8
> 0x40155093   # srai x1, x10, 1
> ```
> Результат в x1=0xFFC т.е -4


</details>

---

### Группа: Load 

Загрузка из памяти (opcode = 0000011)

Здесь мы **читаем** данные из ОЗУ/ПЗУ по адресу `(rs1 + imm)` и кладем их в `rd`.

RISC-V использует **байтовую** адресацию памяти, т.е. адрес `(rs1 + imm)` это индекс ячейки памяти из расчета что одна ячейка занимает 8 бит.

| Команда   | funct3 | Что делает |
| :---      | :---   | :---       |
| **`lb`**  | `000`  | Load **Byte**: загружает 1 байт (8 бит) и **расширяет по знаку** до 32 бит    |
| **`lbu`** | `100`  | Load **Byte Unsigned**: загружает 1 байт и **заполняет нулями** старшие биты  |
| **`lh`**  | `001`  | Load **Halfword**: загружает 2 байта (16 бит) со знаком                       |
| **`lhu`** | `101`  | Load Halfword Unsigned: загружает 2 байта без знака                           |
| **`lw`**  | `010`  | Load **Word**: загружает полные 32 бита (4 байта)                             |


Поле `funct3` для типа LOAD уже содержит в себе всю информацию о размере и знаковости:
* 000 — lb (байт, со знаком)
* 001 — lh (полуслово, со знаком)
* 010 — lw (слово, 32 бита)
* 100 — lbu (байт, без знака)
* 101 — lhu (полуслово, без знака)

В инструкции `lb` запись `offset(rs1)` — это полноценный, сквозной адрес конкретного байта в единой большой памяти, а не смещение внутри одной 32-битной ячейки. Когда АЛУ складывает `rs1 + offset`, оно получает один единственный плоский адрес. Например, адрес offset(rs1)=11, для процессора адрес 11 означает: «Мне нужен 11-й по счету байт от начала памяти». Для него вся память — это просто бесконечная лента из одиночных байтов. Поэтому модуль `LoadStoreUnit` это 32 битная ОЗУ с 8 битной адресацией, и склеивания байтов в слово, полуслово.



```
0x4030201

00000100 00000011 00000010 00000001

                                          split word
                                              |
| 3-й        | 2-й      | 1-й      | 0-й      | 7-й      | 6-й      | 5-й      | 4-й         | byte addr
| 00000100   | 00000011 | 00000010 | 00000001 | 00000100 | 00000011 | 00000010 | 00000001    |
|----------------0x4030201--------------------|-----------------0x4030201--------------------| 4 byte lw addr=0-й и 4-й 0x04030201
|----------0x1040302 (1)-----------|                                           |0x1040302 (2)| 4 byte lw addr=1-й 0x1040302  00000001_00000100_00000011_00000010
|-----0x3020104 (1)-----|                                           |--------0x3020104 (2)---| 4 bute lw addr=2-й 0x2010403  00000010_00000001_00000100_00000011
|0x3020104(1)|                                           |----------0x3020104(2)-------------| 4 byte lw addr=3-й 0x3020104  00000011_00000010_00000001_00000100


|                       |----------0x201------|                                              | 2 byte lh addr=0-й  00000010_00000001
|            |---------0x302 ------|                                                         | 2 byte lh addr=1-й  00000011_00000010
|-------0x403-----------|                                                                    | 2 byte lh addr=2-й  00000100_00000011
|  0x104 (1) |                                                                 |  0x104 (2)  | 2 byte lh addr=3-й  00000001_00000100
|                                                                   |-----------0x201--------| 2 byte ln addr=4-й  00000010_00000001

```


<br>
<details>
<summary> <b> # 10 Instruction: lb </b> </summary>

Инструкция чтения `lb` (**L**oad **B**yte) загружает 1 байт из оперативной памяти расширяя знак до 32 бит, результат записывает в регистр.
 
До этого момента мы работали с памятью инструкций, где каждая ячейка — это монолитное 32-битное слово, и адрес всегда прыгал сразу на +4. Но в иструкции `lb` мы используем побайтный доступ, а не 4-х байтный слова.

Так как в компьютерной архитектуре **побайтная адресация**, принято правило: `1 адрес в памяти = 1 байт (8 бит)`. Поэтому каждое 32 битное слово имеет 4 адреса для своих ячеек по 1 байту (8 бит) каждая.

Например, 32-х битное число 33282 (decimal), хранится по адресу 0x0 но распределенно в памяти по байту (8 бит). Для доступа к части этого слова используется инструкция `lb`:
```  
 0x4      0x3      0x2      0x1      0x0
0xF4 00000000 00000000 10000010 00000010
```

Оперативная память находится в DataMemory (RAM), сюда CPU сам пишет и сам читает. Для тестирования можно вручную записать нужные данные по расчетному адресу либо использовать инструкцию `sw` (store word).
 

Получив значение по новому адресу, нам нужно его расширить до 32 битного значения, а для этого нужно узнать его знак.
Чтобы сохранить математический смысл отрицательного числа, процессор обязан узнать: исходное число в байте было отрицательным или положительным? Единственный способ это узнать — посмотреть на самый старший (7-й) бит этого 8-битного числа. Если там 1 (число было отрицательным), процессор искусственно забивает старшие 24 бита регистра единицами.

  
ALU вычисляет новый адрес так, на вход A идут данные с rs1 (source), а на вход B идут данные с imm (offset) их значения складываются (ALU ADD):
* Если новый адрес `offset(rs1)=0x0`, там хранится значение `00000010`, у него старший бит 0, значит, блок знакового расширения дописывает слева 24 нуля: `00000000 00000000 00000000 00000010`. Итог в регистре: `0x00000002` (десятичное 2).
* Если новый адрес `offset(rs1)=0x1`, там хранится значение `10000010`, у него старший бит 1, блок знакового расширения дублирует эту единицу влево 24 раза, заполняя всё старшее пространство регистра: `11111111 11111111 11111111 10000010`. Итог в регистре: `0xFFFFFF82` (десятичное -126).
 
 
Посчитать новый адрес, к значению из `x10` прибавить 1. По новому адресу взять байт и расширить его с учетом знака до размера слова в 4 байта (32 бита) и сохранить результат в `x5`

```
lb rd, offset(rs1)
---
lb x5, 1(x10)
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа) offset  |
| ---    | ---                   | ---                    | ---                     |
| 7 бит  | 5 бит                 | 5 бит                  | 12 бит                  |
| lb     | x5                    | x10                    | 1                       |


```
hexadecimal  : 0x00150283
Assembly     : lb x5, 0x1(x10)
Format       : I (LOAD)
instr        : 00000000000101010000001010000011
opcode       : .........................0000011 (0x3)
rd (receiver): ....................00101....... x5 ("t0")
funct3       : .................000............ (lb)
rs1 (source) : ............01010............... x10 ("a0")
imm          : 000000000001.................... raw=0x1 signed=1 decimal
```
 
> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00400513   # addi x10, x0, 4
> 0x08200493   # addi x9, x0, 0x82 # 130 decimal если расширить до 12 бит imm
> 0x009500a3   # sb x9, 1(x10) # RAM[5]=0x82
> 0x00150283   # lb x5, 1(x10)
> ```
> Результат в x5=`0xFFFFFF82` (-126 decimal)


Для инструкции lb (Load Byte) адрес offset(rs1)=11 означает: «мне нужен строго один байт, который находится на 11-й позиции в памяти (считая от нуля)».

---

</details>


<br>
<details>
<summary> <b> # 11 Instruction: lbu </b> </summary>

Инструкция чтения `lbu` (**L**oad **B**yte **U**nsigned) загружает 1 байт из оперативной памяти заполняя нулями старшие биты (до 32 бит) и записывает результат в регистр.

Посчитать новый адрес, к значению из `x10` прибавить 1. По новому адресу взять байт и расширить его без учета знака т.е. дополнить нулями до размера слова в 4 байта (32 бита) и сохранить результат в `x5`

```
lbu rd, offset(rs1)
---
lbu x5, 1(x10)
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа) offset  |
| ---    | ---                   | ---                    | ---                     |
| 7 бит  | 5 бит                 | 5 бит                  | 12 бит                  |
| lbu    | x5                    | x10                    | 1                       |



```
hexadecimal  : 0x00154283
Assembly     : lbu x5, 0x1(x10)
Format       : I (LOAD)
instr        : 00000000000101010100001010000011
opcode       : .........................0000011 (0x3)
rd (receiver): ....................00101....... x5 ("t0")
funct3       : .................100............ (lbu)
rs1 (source) : ............01010............... x10 ("a0")
imm          : 000000000001.................... raw=0x1 signed=1 decimal
```

<br>

> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00400513   # addi x10, x0, 4
> 0x08200493   # addi x9, x0, 0x82 # 130 если расширить до 12 бит
> 0x009500a3   # sb x9, 1(x10) # RAM[5]=0x82
> 0x00154283   # lbu x5, 1(x10)
> ```
> Результат в x5=`0x00000082` (130 decimal)


---

</details>


<br>
<details>
<summary> <b> # 12 Instruction: lh </b> </summary>

Инструкция чтения `lh` (**L**oad **H**alfword) загружает 2 байта (16 бит) со знаком и записывает результат в регистр. 

Посчитать новый адрес, к значению из `x10` прибавить 1. По новому адресу взять 2 байта (16 бит) и расширить его с учетом знака до размера слова в 4 байта (32 бита) и сохранить результат в `x5`

```
lh rd, offset(rs1)
---
lh x5, 1(x10)
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа) offset  |
| ---    | ---                   | ---                    | ---                     |
| 7 бит  | 5 бит                 | 5 бит                  | 12 бит                  |
| lh     | x5                    | x10                    | 1                       |


```
hexadecimal  : 0x00151283
Assembly     : lh x5, 0x1(x10)
Format       : I (LOAD)
instr        : 00000000000101010001001010000011
opcode       : .........................0000011 (0x3)
rd (receiver): ....................00101....... x5 ("t0")
funct3       : .................001............ (lh)
rs1 (source) : ............01010............... x10 ("a0")
imm          : 000000000001.................... raw=0x1 signed=1 decimal
```


> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00400513   # addi x10, x0, 4
> 0x20100493   # addi x9, x0, 513 # (0b0000001000000001 16 bit)
> 0x009510a3   # sh x9, 1(x10)
> 0x00151283   # lh x5, 1(x10)
> ```
> Результат в x5=`513` (0x201)


---

</details>


<br>
<details>
<summary> <b> # 13 Instruction: lhu </b> </summary>

Инструкция чтения `lhu` (**L**oad **H**alfword **U**nsigned) загружает 2 байта без знака и записывает результат в регистр. 

Посчитать новый адрес, к значению из `x10` прибавить 1. По новому адресу взять 2 байта (16 бит) и расширить его без учета знака т.е. дополнить нулями до размера слова в 4 байта (32 бита) и сохранить результат в `x5`

```
lhu rd, offset(rs1)
---
lhu x5, 1(x10)
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа) offset  |
| ---    | ---                   | ---                    | ---                     |
| 7 бит  | 5 бит                 | 5 бит                  | 12 бит                  |
| lh     | x5                    | x10                    | 1                       |


```
hexadecimal  : 0x00155283
Assembly     : lhu x5, 0x1(x10)
Format       : I (LOAD)
instr        : 00000000000101010101001010000011
opcode       : .........................0000011 (0x3)
rd (receiver): ....................00101....... x5 ("t0")
funct3       : .................101............ (lhu)
rs1 (source) : ............01010............... x10 ("a0")
imm          : 000000000001.................... raw=0x1 signed=1 decimal
```


> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0xdff00293  # addi x5, x0, 0xFDFF (-513 0b1111110111111111) 
> 0x00501023  # sh x5, 0(x0) (RAM[0]=0b11111111 RAM[1]=0b11111101)
> 0x00005303  # lhu x6, 0(x0)
> ```
> Результат в x6=`-513` (0xFDFF)


---

</details>


<br>
<details>
<summary> <b> # 14 Instruction: lw </b> </summary>

Инструкция чтения `lw` (**L**oad **W**ord) - загружает полные 32 бита (4 байта) и записывает результат в регистр. 

Посчитать новый адрес, к значению из `x10` прибавить 1. По новому адресу взять из памяти все 4 байта (32 бита) и сохранить результат в `x5`

```
lw rd, offset(rs1)
---
lw x5, 1(x10)
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | imm (константа) offset  |
| ---    | ---                   | ---                    | ---                     |
| 7 бит  | 5 бит                 | 5 бит                  | 12 бит                  |
| lw     | x5                    | x10                    | 1                       |


```
Instr hex    : 0x00002303
Assembly     : lw x6, 0x0(x0)
Format       : I (LOAD)
instr        : 00000000000000000010001100000011
opcode       : .........................0000011 (0x3)
rd (receiver): ....................00110....... x6 ("t1")
funct3       : .................010............ (lw)
rs1 (source) : ............00000............... x0 ("zero")
imm          : 000000000000.................... raw=0x0 signed=0 decimal
```

> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x040302b7 # lui x5, 0x04030  # 0x4030 01000000001100000000000000000000
> 0x00502023 # sw x5, 0(x0)     # RAM[0]=0x40300000 
> 0x00002303 # lw x6, 0(x0)
> ```
> Результат в x6=0x40300000

 

</details>

---

### Группа: JALR

Инструкция управления потоком выполнения.

<br>
<details>
<summary> <b> # 15 Instruction: jalr </b> </summary>


Инструкция `jalr` (**J**ump **a**nd **L**ink **R**egister) — это абсолютный (косвенный) безусловный переход, что означает адрес для прыжка не зависит от места вызова инструкции, а расчитывается взятием адреса из регистра плюс смещение `imm`. 

> А инструкция [jal](#Инструкции-j-типа) это **относительный** (прямой) безусловный переход, что означает адрес для прыжка жестко «привязан» к месту, где написана сама инструкция т.е. относительно текущего вызова команды.

Вычисляет целевой адрес как сумму регистра-источника `rs1` и знакового 12-битного непосредственного значения `imm`, за­тем выполняет переход по этому адресу и сохраняет адрес следующей инст­рукции в регистре-приемнике `rd`. 

Она незаменима для возврата из функций или вызова функций по указателю.

Когда ей предшествует `аиірс`, инструкция `jalr` может выполнить переход относительно PC в любое место 32-битного ад­ресного пространства.
 
```
jalr rd, offset(rs1)
---
jalr x1, 0x9(x5)
```

> Псевдоинструкция `ret` транслируется как `jalr x0, x1, 0`. Инструкция `jalr` обычно записывает `PC + 4` в регистр `rd`, но поскольку `rd = x0`, эта запись игнорируется. Затем процессор загружает в `PC` значение `x1 + 0` (то есть содержимое регистра `ra`), тем самым выполняя возврат из функции.


| 31–20 (12 бит)     |  19–15 (5 бит)         | 14–12 (3 бита) |    11–7 (5 бит)       | 6–0 (7 бит) |
| :------------:     | :-------------:        | :------------: | :----------------:    | :---------: |
| `imm[11:0]` offset | rs1 (регистр-источник) |     funct3     | rd (регистр-приёмник) |    opcode   |
|        `0x9`       |           `x5`     m   |      `000`     |          `x1`         |  `1100111`  |


В инструкция `jalr` адрес приходит из регистра `rs1`, который может содержать любое значение, поэтому спецификация требует принудительно сбросить младший бит перед записью в PC. Потому что инструкции в RISC-V всегда выровнены по 2 байтам (или 4), и нечётный адрес физически не может быть адресом начала инструкции, поэтому младший бит всегда игнорируется и обнуляется.

```
tmp = pc + 4
pc  = (rs1 + imm) & ~1 # принудительное обнуление младшего бита адреса
rd  = tmp
```

```
Instr hex    : 0x009380e7
Assembly     : jalr x1, 0x9(x7)
Format       : I (JALR)
instr        : 00000000100100111000000011100111
opcode       : .........................1100111 (0x67)
rd (receiver): ....................00001....... x1 ("ra")
funct3       : .................000............ (jalr)
rs1 (source) : ............00111............... x7 ("t2")
imm          : 000000001001.................... raw=0x9 signed=9 decimal
```


> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x0        # PC=0
> 0x00b00393 # PC=4     # addi x7, x0, 0xB 
> 0x00300193 # PC=8     # addi x3, x0, 3
> 0x009380e7 # PC=0xC   # jalr x1, 0x9(x7)  # JUMP START, addr PC 0xB+0x9=0x14 (20 dec) 
> 0x00418193 # PC=0x10  # addi x3, x3, 4
> 0x00218193 # PC=0x14  # addi x3, x3, 2    # JUMP TARGET
> ```
> Результат в:
> * x1=0x10 (0xC + 4 = 0x10, адрес возврата - это следующая инструкция после адреса инструкции с прыжком)
> * x3=0x5 (3+2=5 результат не 3+4+2=9 так как инструкцию 0x00418193 мы перепрыгнули)
 
 
 

</details>

---

## Инструкции System
 

| Инструкция | `imm[11:0]=instr[31:20]`     | funct3  | rs1 (регистр-источник) | rd (регистр-приёмник)| opcode  | кодируется  |
| ---------- | --------------               | ------- |---                     | ---                  | ---     | ---         |
| ecall      | `000000000000`               | 000     | 0                      |  0                   | 1110011 | 0x00000073  |
| ebreak     | `000000000001`               | 000     | 0                      |  0                   | 1110011 | 0x00100073  |

<br>
<details>
<summary> <b> # 16 Instruction: ecall </b> </summary>

Инструкция `ecall` (**E**nvironment **C**all in Machine mode, exception code = 11) — это специальная системная команда, которая используется для генерации программного исключения (System Call / Syscall).

Её главная задача — передать управление из пространства пользователя (User Mode, где работает обычная программа) в пространство операционной системы или супервизора (Kernel/Machine Mode) для выполнения привилегированных операций. 

Обычная программа пользователя не имеет прямого доступа к оборудованию: она не может сама вывести символ на экран, записать файл на диск или выделить память. Всё это контролирует операционная система (или среда симуляции). Когда программе нужно сделать что-то из этого списка, она выполняет `ecall`. Процессор мгновенно останавливает выполнение обычного кода и прыгает на специальный адрес операционной системы — обработчик исключений (Trap Handler).

Сама инструкция `ecall` не имеет операндов (у неё нет полей `rs1`, `rs2` или `rd`). Чтобы объяснить операционной системе, какую именно задачу нужно выполнить, программист перед вызовом `ecall` загружает аргументы в стандартные регистры по общепринятому соглашению (ABI):

 


**1.** Программная подготовка к вызову ecall:
* в startup коде записывается адрес общего обрабочика всех прерываний в `mtvec`. При возникновении trap общий обработчик по номеру причины из mcause, решает на какой конкретно адрес перебросить для обработки trap  
* записать в регистры `a0-a7` передаваемые параметры для обработчика
   * в регистр `a7` (`x17`) записывается номер системного вызова (например: 1 — напечатать целое число, 4 — напечатать строку, 10 — завершить программу).
* вызвать `ecall`

**2.** Аппаратная работа процессора после вызова `ecall`:
* mstatus.MPP = текущий_режим;  // например, если вы были в U-режиме, MPP = 0b00
* mstatus.MIE = 0;              // отключил прерывания (глобально)
* запись в регистр `mcause` причины вызова, согласно текщего режима работы процессора
  * если ecall выполнен в U-режиме (User Mode), в mcause будет записано значение 8 (Environment call from U-mode)
  * если ecall выполнен в S-режиме (Supervisor Mode), в mcause будет записано значение 9 (Environment call from S-mode)
  * если ecall выполнен в M-режиме (Machine Mode), в mcause будет записано значение 11 (Environment call from M-mode)
* запись в регистр `mepc` адреса вызова ecall т.е. текущая позиция PC
* Сохраняет текущий режим в mstatus.MPP
* Отключает прерывания (mstatus.MIE = 0)
* переход по адресу обработчика из регистра `mtvec`

**3.** Программный выход из обработчика:
* сохраняете контекст (регистры на стек)
* читаете `mcause`, чтобы понять, что произошло
* инкрементировать на 4 байта адрес в `mepc` (что бы перейти на след. инструкцию после `ecall`)
* обрабатываем системный вызов (параметры в регистрах `a0-a7`)
* восстанавление контекста
* вызов `mret` возврат из обработчика 

**4.** Аппаратная работа процессора после вызова `mret`:
```
mstatus.MIE = mstatus.MPIE;   // Восстанавливаем старый MIE
mstatus.MPIE = 1;             // Устанавливаем в 1 (запасное значение)
mstatus.MPP = 0b00;           // Сбрасываем MPP в U-режим (или 0)
pc = mepc;                    // Прыгаем на адрес возврата
```

 

---


   
Для тестирования инструкции ecall, необходимо реализовать расширение [Zicsr](#zicsr-расширение-control-and-status-registers-csr) 

Минимальный тест проверяет: что процессор умеет корректно переходить в обработчик по `mtvec`, сохранять адрес возврата в `mepc`, а затем возвращаться через `mret` к инструкции после `ecall`.

**Файл test_ecall.S**
```
.section .text
.global _start

_start:

    # Адрес обработчика исключений
    la t0, trap_handler
    csrw mtvec, t0

    # Печатаем 'A'
    li t0, 'A'
    li t1, 0x10000000
    sb t0, 0(t1)

    # Вызываем исключение
    ecall

    # После возврата из обработчика
    li t0, 'B'
    sb t0, 0(t1)

loop:
    j loop


##################################################
# Обработчик исключений
##################################################

trap_handler:

    # Печатаем 'E'
    li t0, 'E'
    li t1, 0x10000000
    sb t0, 0(t1)

    # mepc = mepc + 4
    csrr t2, mepc
    addi t2, t2, 4
    csrw mepc, t2

    # Вернуться
    mret
```    

**Компиляция test_ecall.S**

riscv64-unknown-elf-gcc -march=rv32i_zicsr -mabi=ilp32 -nostdlib -T linker.ld test_ecall.S -o test_ecall.elf

(компиляция с флагом расширения Zicsr и без файла startup.S)

**Создание чистого бинарного файла (flat binary)**

riscv64-unknown-elf-objcopy -O binary test_ecall.elf test_ecall.bin

**Создание hex-dump**  

(echo "v2.0 raw"; hexdump -v -e '1/4 "0x%08x\n"' test_ecall.bin) > test_ecall.hex

**Файл test_ecall.hex**
```
v2.0 raw
0x00000297 # PC=0x0  # auipc x5, 0         # формируем адрес обработчика trap_handler
0x02828293 # PC=0x4  # addi x5, x5, 40     ## адрес trap_handler x5=0x28
0x30529073 # PC=0x8  # csrrw x0, mtvec, x5 ## mtvec=0x28 сюда процессор прыгнет при ecall
0x04100293 # PC=0xC  # addi x5, x0, 65     # подготовка к print A (65='A')
0x10000337 # PC=0x10 # lui x6, 65536       ## получаем адрес UART
0x00530023 # PC=0x14 # sb x5, 0(x6)        ## записывает RAM[0x10000000]='A' и UART печатает 'A'
0x00000073 # PC=0x18 # ecall               # вызов исключения и переход по адресу PC=0x28
0x04200293 # PC=0x1C # addi x5, x0, 66     # срузу после выхода из исключения, подготовка к print B (66='B')
0x00530023 # PC=0x20 # sb x5, 0(x6)        ## записывает RAM[0x10000000]='B' и UART печатает 'B'
0x0000006f # PC=0x24 # jal x0, 0           # вечный цикл на месте
0x04500293 # PC=0x28 # addi x5, x0, 69     # trap_handler, подготовка к print E (69='E')
0x10000337 # PC=0x2C # lui x6, 65536       ## получаем адрес UART
0x00530023 # PC=0x30 # sb x5, 0(x6)        ## записывает RAM[0x10000000]='E' и UART печатает 'E'
0x341023f3 # PC=0x34 # csrrs x7, mepc, x0  ## читаем mepc, x7=mepc=0x18
0x00438393 # PC=0x38 # addi x7, x7, 4      ## увеличиваем адрес возврата +4, x7=0x1C
0x34139073 # PC=0x3C # csrrw x0, mepc, x7  ## записываем обратно, mepc=0x1C
0x30200073 # PC=0x40 # mret                ## возвращаемся, PC=mepc=0x1C

# Результат: вывод терминала - AEB
```


Подготовка к исключению:
* формируем адрес обработчика `trap_handler` и записываем его в `mtvec` (это только на этапе разработки. В реализованном варианте процессора, startup код уже сохраняет адрес общего обработчика в `mtvec`, а внутри общего обработчика происходит перенаправление на конкретный обработчик согласно номеру причины trap из `mcause`)
* вызов ecall

```
0x30529073 # csrrw x0, mtvec, x5  
```

Выход из исключения:
* прочитать адрес возврата в mepc и увеличить его на 4 и сохранить обратно в mepc
* вызов mret

```
0x341023f3 # PC=0x34 # csrrs x7, mepc, x0  # читаем mepc, x7=mepc=0x18
0x00438393 # PC=0x38 # addi x7, x7, 4      # увеличиваем адрес возврата +4, x7=0x1C
0x34139073 # PC=0x3C # csrrw x0, mepc, x7  # записываем обратно, mepc=0x1C
0x30200073 # PC=0x40 # mret                # возвращаемся, PC=mepc=0x1C
```



---

</details>


<br>
<details>
<summary> <b> # 17 Instruction: ebreak </b> </summary>

Инструкция `ebreak` (**E**nvironment **B**reak, exception code = 3) — это инструкция для отладки (не для работы программы), ее смысл остановить процессор и передай управление отладчику.
 
 
<br>
<details>
<summary><b>Тестирование ebreak</b> </summary>
 
Для проверки работы инструкции ebreak, необхода реализация [CSR](#zicsr-расширение-control-and-status-registers-csr) 

**Файл test_ebreak.S**
```
.section .text
.global main

main:
    
    # Печатаем 'A'
    li t0, 0x41 # 65 'A'
    li t1, 0x10000000
    sb t0, 0(t1)

    # Вызываем исключение
    ebreak

    # После возврата из обработчика
    li t0, 'B'
    sb t0, 0(t1)
     
loop:
    j main
 
```    

**Компиляция test_ebreak.S**
```
riscv64-unknown-elf-gcc -march=rv32i_zicsr -mabi=ilp32 -nostdlib -T linker.ld startup.S trap.S test_ebreak.S -o test_ebreak.elf
```

**Создание чистого бинарного файла (flat binary)**
```
riscv64-unknown-elf-objcopy -O binary test_ebreak.elf test_ebreak.bin
```

**Создание hex-dump**  
```
(echo "v2.0 raw"; hexdump -v -e '1/4 "0x%08x\n"' test_ebreak.bin) > test_ebreak.hex
```
 
В ROM можно загрузить test_ebreak.bin или test_ebreak.hex.

**Дизассемблировать все секции с кодом**
```
riscv64-unknown-elf-objdump -D test_ebreak.elf

---


0000007c <trap_handler>:
  7c:   342022f3                csrr    t0,mcause
  80:   80000337                lui     t1,0x80000
  84:   0062f3b3                and     t2,t0,t1
  88:   06039a63                bnez    t2,fc <irq_handler>
  8c:   00b00313                li      t1,11
  90:   00628c63                beq     t0,t1,a8 <ecall_handler>
  94:   00300313                li      t1,3
  98:   02628663                beq     t0,t1,c4 <ebreak_handler>
  9c:   00200313                li      t1,2
  a0:   04628063                beq     t0,t1,e0 <illegal_handler>

000000a4 <hang>:
  a4:   0000006f                j       a4 <hang>

000000a8 <ecall_handler>:
  a8:   04500293                li      t0,69
  ac:   10000337                lui     t1,0x10000
  b0:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffed8>
  b4:   341023f3                csrr    t2,mepc
  b8:   00438393                addi    t2,t2,4
  bc:   34139073                csrw    mepc,t2
  c0:   30200073                mret

000000c4 <ebreak_handler>:
  c4:   04b00293                li      t0,75
  c8:   10000337                lui     t1,0x10000
  cc:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffed8>
  d0:   341023f3                csrr    t2,mepc
  d4:   00438393                addi    t2,t2,4
  d8:   34139073                csrw    mepc,t2
  dc:   30200073                mret

000000e0 <illegal_handler>:
  e0:   04900293                li      t0,73
  e4:   10000337                lui     t1,0x10000
  e8:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffed8>
  ec:   341023f3                csrr    t2,mepc
  f0:   00438393                addi    t2,t2,4
  f4:   34139073                csrw    mepc,t2
  f8:   30200073                mret

000000fc <irq_handler>:
  fc:   05100293                li      t0,81
 100:   10000337                lui     t1,0x10000
 104:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffed8>
 108:   30200073                mret

0000010c <main>:
 10c:   04100293                li      t0,65
 110:   10000337                lui     t1,0x10000
 114:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffed8>
 118:   00100073                ebreak
 11c:   04200293                li      t0,66
 120:   00530023                sb      t0,0(t1)


```

</details>


</details>

---

## Инструкции Control


| Команда       | funct3 |   opcode  | Что делает      |
| :---          | :---   |  :---     |:---             |
| **`fence`**   | `000`  | `0001111` | барьер памяти   |
| **`fence.і`** | `001`  | `0001111` | синхронизация между памятью данных (Data RAM) и памятью инструкций (Instruction RAM/ROM)  |
 
<b> # 18 Instruction: fence </b> 

 

Инструкция `fence` — «барьер памяти». Она используется в многоядерных системах или при работе с быстрым DMA-периферией. Она гарантирует, что все операции чтения (Read) и записи (Write) в память, стоящие в коде до FENCE, физически завершатся до того, как начнутся операции чтения/записи, стоящие после FENCE.

Она принимает два операнда: первый определяет типы предыдущих операций обращения к памяти, которые должны завер­шиться до выполнения инструкции `fence`, второй — типы последующих опе­раций обращения к памяти, выполнение которых ограничивает инструкция `fence`. 

Типы операций, упорядочиваемых этой инструкцией, — чтение из па­мяти и запись в память (г и w), а также ввод и вывод через устройства ввода-вывода (і и о). 

Например, инструкция `fence rw, rw `гарантирует, что все опе­рации чтения и записи с использованием адресов памяти, начатые до инст­рукции `fence`, завершатся до начала любых последующих операций чтения из памяти или записи в память. Эта инструкция гарантирует, что любые значе­ния, находящиеся в блоках кеш-памяти процессора, надлежащим образом синхронизированы с памятью или устройством ввода-вывода.

В классическом одноядерном процессоре, где обращения к RAM (инструкции `lw`, `sw`) выполняются строго последовательно, шаг за шагом, то никаких задержек или барьеров создавать не нужно.

В модуле Control Unit при декодировании `fence` все сигналы записи (`RegWrite`, `MemWrite`) выставляются в 0. Процессор просто переходит к `PC + 4`. Инструкция работает как обычный `NOP`.

Выставляя `RegWrite = 0`, `MemWrite = 0` и `Branch = 0` Вы гарантируете, что инструкция пройдет через процессор, не изменив ни состояние регистров, ни данные в памяти, а счетчик команд просто переключится на `PC + 4`

```
Instr hex    : 0x0ff0000f
Assembly     : fence
Format       : Control
instr        : 00001111111100000000000000001111
opcode       : .........................0001111
funct3       : .................000............ (fence)
```            
 
<b> # 19 Instruction: fence.і </b> 

В расширении `Zifencei`
* Z — стандартное (необязательное) расширение.
* i — instruction (инструкции).

`fence.і` — инструкция гарантирует, что все операции сохранения в памяти инструкций будут завершены до выполнения инструкции `fence.і`. Данная ин­струкция в основном полезна в контексте самомодифицирующегося кода.

```
Instr hex    : 0x0ff0100f
Assembly     : fence.і
Format       : Control
instr        : 00001111111100000001000000001111
opcode       : .........................0001111
funct3       : .................001............ (fence.і)
```

Реализуем как `NOP` (так как однопроходной процессор без кэша инструкций и без предварительной выборки инструкций).

---

## Инструкции R-типа 
(Register-Register Arithmetic)
 
*Применяется для операций между двумя регистрами (например: `add`, `sub`, `and`, `or`, `slt`). Здесь нет константы, но появляется второй регистр-источник `rs2` и дополнительное поле `funct7`*

| funct7    | rs2 (2-й регистр-источник) | rs1 (1-й регистр-источник) | funct3 (подфункция) | rd (регистр-приёмник) | opcode  |
| ---       | ---                        | ---                        | ---                 | ---                   | ---     |
| **31:25** | **24:20**                  | **19:15**                  | **14:12**           | **11:7**              | **6:0** |
| 7 бит     | 5 бит                      | 5 бит                      | 3 бита              | 5 бит                 | 7 бит   |


`funct7` (биты 31–25): Дополнительный код. Чаще всего он состоит из нулей, но, например, у команды вычитания `sub` или арифметического сдвига `sra` в этом поле один из битов становится единицей (`0100000`), что позволяет отличить их от `add` и `srl`.

Величина сдвигов (`sll`, `srl`, `sra`): Поскольку регистр имеет размер 32 бита, сдвинуть число можно максимум на 31 позицию. Поэтому ALU при сдвигах берет не все 32 бита из регистра `rs2`, а только его младшие 5 бит — `rs2[4:0]`. Старшие биты регистра `rs2` в операциях сдвига просто игнорируются.


| Команда   | funct3 | funct7    | Что делает                                                              | Пример                            |
| ---       | ---    | ---       | ---                                                                     | ---                               |
| `add`     | `000`  | `0000000` | Сложение двух регистров:<br> `rd = rs1 + rs2`                           | `add x5, x6, x7` → `x5 = x6 + x7` |
| `sub`     | `000`  | `0100000` | Вычитание двух регистров:<br> `rd = rs1 - rs2`                          | `sub x5, x6, x7` → `x5 = x6 - x7` |
| `sll`     | `001`  | `0000000` | Логический сдвиг влево:<br> `rd = rs1 << rs2[4:0]` (младшие 5 бит `rs2`)| `sll x5, x6, x7`                  |
| `slt`     | `010`  | `0000000` | Сравнение на «меньше» со знаком:<br> `rd = (rs1 < rs2) ? 1 : 0`         | `slt x5, x6, x7`                  |
| `sltu`    | `011`  | `0000000` | Сравнение на «меньше» БЕЗ знака:<br> `rd = (rs1 < rs2) ? 1 : 0`         | `sltu x5, x6, x7`                 |
| `xor`     | `100`  | `0000000` | Побитовое XOR:<br> `rd = rs1 ^ rs2`                                     | `xor x5, x6, x7`                  |
| `srl`     | `101`  | `0000000` | Логический сдвиг вправо<br> (забивает старшие биты нулями)              | `srl x5, x6, x7`                  |
| `sra`     | `101`  | `0100000` | Арифметический сдвиг вправо<br> (копирует знаковый бит)                 | `sra x5, x6, x7`                  |
| `or`      | `110`  | `0000000` | Побитовое OR:<br> `rd = rs1 \| rs2`                                     | `or x5, x6, x7`                   |
| `and`     | `111`  | `0000000` | Побитовое AND:<br> `rd = rs1 & rs2`                                     | `and x5, x6, x7`                  |


<br>
<details>
<summary> <b> # 20 Instruction: add </b> </summary>

Инструкция `add` - сложение двух регистров: `rd = rs1 + rs2`

```
add rd, rs1, rs2
---
add x3, x1, x2
```

| opcode | rd (регистр-приёмник) | rs1 (1-й регистр-источник) | rs2 (2-й регистр-источник) |
| ---    | ---                   | ---                        | ---                        |
| 7 бит  | 5 бит                 | 5 бит                      | 5 бит                      |
| add    | x3                    | x1                         | x2                         |
 
```
hexadecimal  : 0x002081B3
Assembly     : add x3, x1, x2
Format       : R
instr        : 00000000001000001000000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................000............ (add)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
funct7       : 0000000......................... (0)
```
 
> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00200093 # addi x1, x0, 0x2
> 0x00600113 # addi x2, x0, 0x6
> 0x002081b3 # add x3, x1, x2
> ```
> Результат в x3=0x8
 
---

</details>

<br>
<details>
<summary> <b> # 21 Instruction: sub </b> </summary>

Инструкция `sub` - вычитание из регистра `rs1` регистра `rs2`: `rd = rs1 - rs2`

```
sub rd, rs1, rs2
---
sub x3, x1, x2
```

| opcode | rd (регистр-приёмник) | rs1 (1-й регистр-источник) | rs2 (2-й регистр-источник) |
| ---    | ---                   | ---                        | ---                        |
| 7 бит  | 5 бит                 | 5 бит                      | 5 бит                      |
| sub    | x3                    | x1                         | x2                         |


```
hexadecimal  : 0x402081b3
Assembly     : sub x3, x1, x2
Format       : R
instr        : 01000000001000001000000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................000............ (sub)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
funct7       : 0100000......................... (32)
```
 
> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00600093 # addi x1, x0, 0x6
> 0x00200113 # addi x2, x0, 0x2
> 0x402081b3 # add x3, x1, x2
> ```
> Результат в x3=0x4
 
---

</details>

<br>
<details>
<summary> <b> # 22 Instruction: and </b> </summary>

Инструкция `and` - побитовое AND: `rd = rs1 & rs2`

```
and rd, rs1, rs2
---
and x3, x1, x2
```

| opcode | rd (регистр-приёмник) | rs1 (1-й регистр-источник) | rs2 (2-й регистр-источник) |
| ---    | ---                   | ---                        | ---                        |
| 7 бит  | 5 бит                 | 5 бит                      | 5 бит                      |
| and    | x3                    | x1                         | x2                         |

```
hexadecimal  : 0x0020f1b3
Assembly     : and x3, x1, x2
Format       : R
instr        : 00000000001000001111000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................111............ (and)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
funct7       : 0000000......................... (0)
```

> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00600093 # addi x1, x0, 0x6
> 0x00200113 # addi x2, x0, 0x2
> 0x0020f1b3 # and x3, x1, x2
> ```
> Результат в x3=0x2
  
---

</details>

<br>
<details>
<summary> <b> # 23 Instruction: or </b> </summary>

Инструкция `or` - побитовое OR: `rd = rs1 \| rs2`  

```
or rd, rs1, rs2
---
or x3, x1, x2
```

| opcode | rd (регистр-приёмник) | rs1 (1-й регистр-источник) | rs2 (2-й регистр-источник) |
| ---    | ---                   | ---                        | ---                        |
| 7 бит  | 5 бит                 | 5 бит                      | 5 бит                      |
| or     | x3                    | x1                         | x2                         |

```
hexadecimal  : 0x0020e1b3
Assembly     : or x3, x1, x2
Format       : R
instr        : 00000000001000001110000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................110............ (or)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
funct7       : 0000000......................... (0)
```

> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00600093 # addi x1, x0, 0x6
> 0x00100113 # addi x2, x0, 0x1
> 0x0020e1b3 # or x3, x1, x2
> ```
> Результат в x3=0x7
  
---

</details>

<br>
<details>
<summary> <b> # 24 Instruction: xor </b> </summary>

Инструкция `xor` - побитовое XOR: `rd = rs1 ^ rs2` 

```
xor rd, rs1, rs2
---
xor x3, x1, x2
```

| opcode | rd (регистр-приёмник) | rs1 (1-й регистр-источник) | rs2 (2-й регистр-источник) |
| ---    | ---                   | ---                        | ---                        |
| 7 бит  | 5 бит                 | 5 бит                      | 5 бит                      |
| xor    | x3                    | x1                         | x2                         |

```
hexadecimal  : 0x0020c1b3
Assembly     : xor x3, x1, x2
Format       : R
instr        : 00000000001000001100000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................100............ (xor)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
funct7       : 0000000......................... (0)
```

> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00500093 # addi x1, x0, 0x5
> 0x00600113 # addi x2, x0, 0x6
> 0x0020c1b3 # xor x3, x1, x2
> ```
> Результат в x3=0x3
  
---

</details>


<br>
<details>
<summary> <b> # 25 Instruction: sll </b> </summary>
 
Инструкция `sll` (**S**hift **L**eft **L**ogical) выполняет логический сдвиг влево значения в `rs1` на заданную величину из `rs2`, результат пишется в `rd`

```
sll rd, rs1, rs2
---
sll x3, x1, x2
```

Так как регистры 32-х битные, то для величины сдвига установленно ограничение в 5 бит. Число хранимое в регистре для сдвига `rs2` использует только 5 младших бит (0b00011111 0x01F)


```
shamt = rs2 & 0x1F
rs1 << shamt = rd

x1 << x2 = x3
2 << 1   = 4
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | rs2 (регистр-источник shamt) |
| ---    | ---                   | ---                    | ---                          |
| 7 бит  | 5 бит                 | 5 бит                  | 5 бит                        |
| sll    | x3                    | x1                     | x2                           |

```
hexadecimal  : 0x002091b3
Assembly     : sll x3, x1, x2
Format       : R
instr        : 00000000001000001001000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................001............ (sll)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
funct7       : 0000000......................... (0)
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0x00200093   # addi x1, x0, 2
> 0x00100113   # addi x2, x0, 1
> 0x002091b3   # sll x3, x1, x2
> ```
> Результат в x3=4

---

</details>


<br>
<details>
<summary> <b> # 26 Instruction: srl </b> </summary>

После сдвига, значение в регистре `rd` заполняется нулями до величины 32 бит.

Инструкция `srl` (**S**hift **R**ight **L**ogical) выполняет логический сдвиг вправо значения в `rs1` на заданную величину из `rs2`, результат пишется в `rd`

```
srl rd, rs1, rs2
---
srl x3, x1, x2
```

Так как регистры 32-х битные, то для **величины** сдвига (значения регистра) установленно ограничение в 5 бит. Число хранимое в регистре для сдвига `rs2` использует только 5 младших бит (0b00011111 0x01F)

```
shamt = rs2 & 0x1F
rs1 >> rs2   = rd

x1 >> x2     = x3
6  >> 1      = 3
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | rs2 (регистр-источник shamt) |
| ---    | ---                   | ---                    | ---                          |
| 7 бит  | 5 бит                 | 5 бит                  | 5 бит                        |
| rs1    | x3                    | x1                     | x2                           |


```
hexadecimal  : 0x0020d1b3
Assembly     : srl x3, x1, x2
Format       : R
instr        : 00000000001000001101000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................101............ (srl)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
funct7       : 0000000......................... (0)
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0x00600093   # addi x1, x0, 6
> 0x00100113   # addi x2, x0, 1
> 0x0020d1b3   # srl x3, x1, x2
> ```
> Результат в x3=3


---

</details>


<br>
<details>
<summary> <b> # 27 Instruction: sra </b> </summary>

После сдвига, значение в регистре `rd` заполняется знаком старшего бита до величины 32 бит.

Инструкция `sra` (**S**hift **R**ight **A**rithmetic) выполняет арифметический сдвиг вправо значения в `rs1` на заданную величину из `rs2`, результат пишется в `rd`.

Так как регистры 32-х битные, то для **величины** сдвига (значения регистра) установленно ограничение в 5 бит. Число хранимое в регистре для сдвига `rs2` использует только 5 младших бит (0b00011111 0x01F)

> старшие биты (слева) заполняет битом знака (MSB)

```
sra rd, rs1, rs2  
---
sra x3, x1, x2
```

```
shamt = rs2 & 0x1F 
rs1 >> rs2   = rd

x1 >> x2     = x3
-8 >> 1      = -4
```

| opcode | rd (регистр-приёмник) | rs1 (регистр-источник) | rs2 (регистр-источник shamt) |
| ---    | ---                   | ---                    | ---                          |
| 7 бит  | 5 бит                 | 5 бит                  | 5 бит                        |
| sra    | x3                    | x1                     | x2                           |


```
hexadecimal  : 0x4020d1b3
Assembly     : sra x3, x1, x2
Format       : R
instr        : 01000000001000001101000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................101............ (sra)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
funct7       : 0100000......................... (32)
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0xff800093   # addi x1, x0, 0xFFFFFFF8 # -8
> 0x00100113   # addi x2, x0, 1
> 0x4020d1b3   # sra x3, x1, x2
> ```
> Результат в x3=0xFFFC т.е -4 decimal

---

</details>


<br>
<details>
<summary> <b> # 28 Instruction: slt </b> </summary>

Инструкция `slt` (**S**et **L**ess **T**han) - сравнение на «меньше» со знаком: `rd = (rs1 < rs2) ? 1 : 0`

Выполняет сравнение со знаком, если значение в регистре-источнике `rs1` меньше чем в `rs2` то в целевой регистр-приёмник `rd` запишется 1, иначе 0.

```
slt rd, rs1, rs2
---
slt x3, x1, x2
```

```
x3 = x1 < x2? 1:0;
```

| opcode | rd (регистр-приёмник) | rs1 (1-й регистр-источник)| rs2 (2-й регистр-источник) |
| ---    | ---                   | ---                       | ---                        |
| slt    | x3                    | x1                        | x2                         |


```
hexadecimal  : 0x0020a1b3
Assembly     : slt x3, x1, x2
Format       : R
instr        : 00000000001000001010000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................010............ (slt)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
funct7       : 0000000......................... (0)
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0xfff00093   # addi x1, x0, -1
> 0x00200113   # addi x2, x0, 2 
> 0x0020a1b3   # slt x3, x1, x2
> ```
> Результат в x3=1
 
---

</details>

<br>
<details>
<summary> <b> # 29 Instruction: sltu </b> </summary>

Инструкция `sltu` (**S**et **L**ess **T**han **U**nsigned) сравнение на «меньше» без знака: `rd = (rs1 < imm) ? 1 : 0`

Выполняет сравнение без знака, если значение в регистре-источнике `rs1` меньше чем в `rs2` то в целевой регистр-приёмник `rd` запишется 1, иначе 0.

```
sltu rd, rs1, rs2
---
sltu x3, x1, x2
```

```
x3 = x1 < x2? 1:0;
```

| opcode | rd (регистр-приёмник) | rs1 (1-й регистр-источник) | rs2 (2-й регистр-источник) |
| ---    | ---                   | ---                        | ---                        |
| sltu   | x3                    | x1                         | x2                         |

```
hexadecimal  : 0x0020b1b3
Assembly     : sltu x3, x1, x2
Format       : R
instr        : 00000000001000001011000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................011............ (sltu)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
funct7       : 0000000......................... (0)
```

> [!EXAMPLE]
> ROM:
> ```
> v2.0 raw
> 0xfff00093   # addi x1, x0, -1 #  -1 = 0xFFFFFFFF = 4294967295 unsigned
> 0x00200113   # addi x2, x0, 2 
> 0x0020b1b3   # sltu x3, x1, x2
> ```
> Результат в x3=0 ( так как 4294967295 > 2)
  
</details>

---

## Инструкции S-типа 

(Store)

*Применяется для **записи** данных в память (например: `sw`, `sh`, `sb`). Регистр-приёмник `rd` здесь не нужен, а 12-битное смещение `imm` разорвано на две части (7 бит и 5 бит), чтобы регистры `rs1` и `rs2` оставались на тех же позициях, что и в R-типе.*

RISC-V использует **байтовую** адресацию памяти, т.е. адрес `(rs1 + imm)` это индекс ячейки памяти из расчета что одна ячейка занимает 8 бит.

Способ извлечения константы (Immediate) из кода инструкции: `imm[11:5]` и `imm[4:0]` (12 бит)

|   31–25 (7 бит)   |  24–20 (5 бит) | 19–15 (5 бит) | 14–12 (3 бита) |    11–7 (5 бит)   | 6–0 (7 бит) |
| :---------------: | :------------: | :-----------: | :------------: | :---------------: | :---------: |
|   **imm[11:5]**   |     **rs2**    |    **rs1**    |   **funct3**   |    **imm[4:0]**   |  **opcode** |
| старшая часть imm | регистр-данные | базовый адрес | операция store | младшая часть imm |  `0100011`  |


Чтобы получить 32-битное смещение константы, процессор склеивает их в правильном порядке: `imm[11] + imm[10:5] + imm[4:0]`, делает знаковое расширение (как в I-типе): дублирует бит 31 во все старшие биты.
 

| Команда  | opcode    | funct3 | Что делает                                                                             |
| :---     | :---      | :---   | :---                                                                                   |
| **`sb`** | `0100011` | `000`  | **Store Byte**: сохраняет младший байт (8 бит) из `rs2` в память по адресу `rs1 + imm` |
| **`sh`** | `0100011` | `001`  | **Store Halfword**: сохраняет младшие 2 байта (16 бит) из `rs2`                        |
| **`sw`** | `0100011` | `010`  | **Store Word**: сохраняет все 4 байта (32 бита) из `rs2`                               |


```
0x4030201

00000100 00000011 00000010 00000001

                                          split word
                                              |
| 3-й        | 2-й      | 1-й      | 0-й      | 7-й      | 6-й      | 5-й      | 4-й         | byte addr
| 00000100   | 00000011 | 00000010 | 00000001 | 00000100 | 00000011 | 00000010 | 00000001    |
|----------------0x4030201--------------------|-----------------0x4030201--------------------| 4 byte lw addr=0-й и 4-й 0x04030201
|----------0x1040302 (1)-----------|                                           |0x1040302 (2)| 4 byte lw addr=1-й 0x1040302  00000001_00000100_00000011_00000010
|-----0x3020104 (1)-----|                                           |--------0x3020104 (2)---| 4 bute lw addr=2-й 0x2010403  00000010_00000001_00000100_00000011
|0x3020104(1)|                                           |----------0x3020104(2)-------------| 4 byte lw addr=3-й 0x3020104  00000011_00000010_00000001_00000100


|                       |----------0x201------|                                              | 2 byte lh addr=0-й  00000010_00000001
|            |---------0x302 ------|                                                         | 2 byte lh addr=1-й  00000011_00000010
|-------0x403-----------|                                                                    | 2 byte lh addr=2-й  00000100_00000011
|  0x104 (1) |                                                                 |  0x104 (2)  | 2 byte lh addr=3-й  00000001_00000100
|                                                                   |-----------0x201--------| 2 byte ln addr=4-й  00000010_00000001

```


<br>
<details>
<summary> <b> # 30 Instruction: sb </b> </summary>

Инструкция записи `sb` (**S**tore **B**yte): сохраняет младший байт (8 бит) из `rs2` в память по адресу `rs1 + imm`

```
sb rs2, offset(rs1)
---
sb x5, 33(x10)
```


| opcode | rs2 (регистр-источник данных)| rs1 (регистр-источник адреса)| imm (константа) offset |
| ---    | ---                          | ---                          | ---                    |
| 7 бит  | 5 бит                        | 5 бит                        | 7+5=12 бит             |
| sb     | x5                           | x10                          | 33                     |


```
hexadecimal  : 0x025500A3
Assembly     : sb x5, 0x021(x10)
Format       : S
instr        : 00000010010101010000000010100011
opcode       : .........................0100011
imm          : 0000001.............00001....... 0b000000100001 raw=0x021 signed=33 decimal
funct3       : .................000............ (sb)
rs1 (source) : ............01010............... x10 ("a0")
rs2 (source) : .......00101.................... x5 ("t0")
```

<br>

> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00900293   # addi x5, x0, 9
> 0x00100513   # addi x10, x0, 1
> 0x025500a3   # sb x5, 0x21(x10) # 0x21=33
> 0x02200083   # lb x1, 0x022(x0)
> ```
> Результат в RAM[34]=9
> Результат в x1=9

Data Path: модуль ControlUnit выставляет флаг ALUSrc, MemWrite и DataMemory готов писать в память. RegFile выдает данные из Rb для DataMemory Din. ALU выдает адре. 

---

</details>

<br>
<details>
<summary> <b> # 31 Instruction: sh </b> </summary>

Инструкция записи `sh` (**S**tore **H**alfword): сохраняет младшие 2 байта (16 бит) из `rs2` в память по адресу `rs1 + imm`
 

```
sh rs2, offset(rs1)
---
sh x5, 33(x10)
```

| opcode | rs2 (регистр-источник данных)| rs1 (регистр-источник адреса)| imm (константа) offset |
| ---    | ---                          | ---                          | ---                    |
| 7 бит  | 5 бит                        | 5 бит                        | 7+5=12 бит             |
| sh     | x5                           | x10                          | 33                     |


```
hexadecimal  : 0x025510A3
Assembly     : sh x5, 0x021(x10)
Format       : S
instr        : 00000010010101010001000010100011
opcode       : .........................0100011
imm          : 0000001.............00001....... 0b000000100001 raw=0x021 signed=33 decimal
funct3       : .................001............ (sh)
rs1 (source) : ............01010............... x10 ("a0")
rs2 (source) : .......00101.................... x5 ("t0")
```


> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x20100293   # addi x5, x0, 513 # 00000010 00000001
> 0x00100513   # addi x10, x0, 1
> 0x025510a3   # sh x5, 0x021(x10) # 0x21=33
> 0x02201083   # lh x1, 0x022(x0)  # 16 byte
> 0x02200103   # lb x2, 0x22(x0)   # first 8 byte
> 0x02300183   # lb x3, 0x23(x0)   # second 8 byte
> ```
> Результат в RAM[34]=1 (8 bit) + RAM[35]=2 (8 bit)
> Результат в:
> * x1=513 (0x201)
> * x2=1 (0b00000001)
> * x3=2 (0b00000010)


---

</details>

<br>
<details>
<summary> <b> # 32 Instruction: sw </b> </summary>

Инструкция записи `sw` (**S**tore **W**ord): сохраняет все 4 байта (32 бита) из `rs2` в память по адресу `rs1 + imm`

```
sw rs2, offset(rs1)
---
sw x5, 33(x10)
```

| opcode | rs2 (регистр-источник данных)| rs1 (регистр-источник адреса)| imm (константа) offset |
| ---    | ---                          | ---                          | ---                    |
| 7 бит  | 5 бит                        | 5 бит                        | 7+5=12 бит             |
| sh     | x5                           | x10                          | 33                     |


```
hexadecimal  : 0x025520A3
Assembly     : sw x5, 0x021(x10)
Format       : S
instr        : 00000010010101010010000010100011
opcode       : .........................0100011
imm          : 0000001.............00001....... 0b000000100001 raw=0x021 signed=33 decimal
funct3       : .................010............ (sw)
rs1 (source) : ............01010............... x10 ("a0")
rs2 (source) : .......00101.................... x5 ("t0")
```
 



> [!EXAMPLE]
> Необходимо собрать 32-х битное число 0x04030201 в регистре x5
>
> 00000100 00000011 00000010 00000001 (67305985 decimal)
>
> ROM:
> ```
> v2.0 raw
> 0x040302b7   # lui x5, 0x04030 
> 0x20128293   # addi x5, x5, 0x201 # x5=0x4030201 (0b00000100_00000011_00000010_00000001) 
> 0x00100513   # addi x10, x0, 1
> 0x025520a3   # sw x5, 0x21(x10)  # 0x21=33 => 0x21 + 0x1 = 0x22 - это адрес для записи 32-х битного 0x4030201
> 0x02202383   # lw x7, 0x22(x0)   # в x7 ожидаем загрузку полного 32-х битного числа 0x4030201
> 0x02201083   # lh x1, 0x22(x0)   # младшие 16 byte
> 0x02401403   # lh x8, 0x24(x0)   # старшие 16 byte
> 0x02200103   # lb x2, 0x22(x0)   # младшие 8 byte
> 0x02300183   # lb x3, 0x23(x0)   # вторые  8 byte
> 0x02400203   # lb x4, 0x24(x0)   # третьи  8 byte 
> 0x02500303   # lb x6, 0x25(x0)   # старшие 8 byte
> 0x02002483   # lw x9, 0x20(x0)   # младшие 16 byte идут в старшие 16 бит итогового 32-х битного числа
> ```
> Результат в RAM[34]=1 (8 bit) + RAM[35]=2 (8 bit)
> Результат в:
> * x7=0x4030201 
> * x1=0x201 (513)
> * x8=0x403 (1027)
> * x2=0x1 (0b00000001)
> * x3=0x2 (0b00000010)
> * x4=0x3 (0b00000011)
> * x6=0x4 (0b00000100)
> * x9=0x0201????

 

</details>

---

## Инструкции B-типа 

Branch, условный относительный переход. Относительный текущего адреса PC.

*Применяется для инструкций ветвления (например: `beq`, `bne`, `blt`). Похож на S-тип, но 12-битное смещение кодирует адрес, кратный 2 байтам (поэтому младший бит `imm[0]` всегда равен 0 и в инструкцию не записывается, а биты перемешаны для упрощения аппаратного знакового расширения).*

Переход при следующих условиях: равно (beq), не равно (bne), меньше чем (bit), меньше чем без знака (bltu), больше или равно (bge), больше или равно без знака (bgeu).

Эти инструкции выполняют указан­ное сравнение между двумя регистрами и, если условие выполнено, передают управление по адресу со смещением, заданным непосредственным 12-битным значением со знаком.

Поскольку внутри 32-битной инструкции под константу `imm` выделено всего 12 бит (которые после добавления 0 в конец превращаются в 13-битное знаковое число), эти инструкции могут перешагнуть максимум на $\pm 4$ КБ от текущей позиции кода.

Способ извлечения константы (Immediate) из кода инструкции: `imm[12] + imm[10:5] + imm[11] + imm[4:1] + 0` (13 бит, младший бит всегда 0)

| imm[12] | imm[10:5] | rs2 (с чем сравниваем) | rs1 (с чем сравниваем) | funct3    | imm[4:1] | imm[11] | opcode |
| ---     | ---       | ---                    | ---                    | ---       | ---      | ---     | ---    |
| 31      | 30:25     | 24:20                  | 19:15                  | 14:12     | 11:8     | 7       | 6:0    |
| 1 бит   | 6 бит     | 5 бит                  | 5 бит                  | 3 бита    | 4 бита   | 1 бит   | 7 бит  |

Константа imm выглядит "рваной" для достижения компромисса между: единым расположением полей регистров `opcode,funct3,rs1,rs1` с другими типами инструкций, компактной 32-битной кодировкой, простотой аппаратного декодера.

<br>

| Команда | funct3 | Что делает                           | Пример               |
| ------- | ------ | ------------------------------------ | -------------------- |
| beq     | 000    | Переход если равно                   | `beq x1, x2, label`  |
| bne     | 001    | Переход если не равно                | `bne x1, x2, label`  |
| blt     | 100    | Переход если меньше (signed)         | `blt x1, x2, label`  |
| bge     | 101    | Переход если больше/равно (signed)   | `bge x1, x2, label`  |
| bltu    | 110    | Переход если меньше (unsigned)       | `bltu x1, x2, label` |
| bgeu    | 111    | Переход если больше/равно (unsigned) | `bgeu x1, x2, label` |


B-type инструкции никогда не записывают адрес возврата, они только изменяют PC, если условие истинно.


<br>
<details>
<summary> <b> # 33 Instruction: beq </b> </summary>

Инструкция `beq` (**B**ranch if **Eq**ual) - если `rs1` равно `rs2`, процессор совершает относительный прыжок на указанный сдвиг (offset) относительно текущего адреса PC.

```
beq rs1, rs2, offset
---
beq x1, x2, 8
```

| opcode | rs1 (с чем сравниваем) | rs2 (с чем сравниваем)| imm (константа) offset               |
| ---    | ---                    | ---                   | ---                                  |
| 7 бит  | 5 бит                  | 5 бит                 | 1+1+6+4=12 бит и +1 бит сами добавим |
| beq    | x1                     | x2                    | 8                                    |


```
PC = if (rs1 == rs2)0 {PC + imm} else {PC + 4};
```

```
Instr hex    : 0x00208463
Assembly     : beq x1, x2, 0x8
Format       : B
instr        : 00000000001000001000010001100011
opcode       : .........................1100011
imm[11]    c : ........................0.......
imm[4:1]   a : ....................0100........
funct3       : .................000............ (beq)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
imm[10:5]  b : .000000.........................
imm[12]    d : 0...............................
imm label    : 0000000001000................... 0x8
part imm     : dcbbbbbbaaaa0...................
```

> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00100213 # PC=0    # addi x4, x0, 1 # accumulate
> 0x00300093 # PC=4    # addi x1, x0, 3
> 0x00300113 # PC=8    # addi x2, x0, 3
> 0x00208463 # PC=0xC  # beq x1, x2, 8  # JUMP START, addr PC 12+8=20 (0xC+0x8=0x14 hex)
> 0x00220213 # PC=0x10 # addi x4, x4, 2
> 0x00120213 # PC=0x14 # addi x4, x4, 1 # JUMP TARGET
>            # PC=0x18
>            # PC=0x1C
> ```
> Результат в:
> * x4=0x2 (1+1=2 результат не 1+2+1=4 так как инструкцию 0x00220213 мы перепрыгнули)
 
 

---

</details>

<br>
<details>
<summary> <b> # 34 Instruction: bne </b> </summary>

Инструкция `bne` (**B**ranch if **N**ot **E**qual) — если `rs1` не равно `rs2`, процессор совершает относительный прыжок на указанный сдвиг (offset) относительно текущего адреса PC.


```
bne rs1, rs2, offset
---
bne x1, x2, 8
```

| opcode | rs1 (с чем сравниваем) | rs2 (с чем сравниваем)| imm (константа) offset               |
| ---    | ---                    | ---                   | ---                                  |
| 7 бит  | 5 бит                  | 5 бит                 | 1+1+6+4=12 бит и +1 бит сами добавим |
| bne    | x1                     | x2                    | 8                                    |


```
PC = if (rs1 != rs2) {PC + imm} else {PC + 4};
```

```
Instr hex    : 0x00209463
Assembly     : bne x1, x2, 0x8
Format       : B
instr        : 00000000001000001001010001100011
opcode       : .........................1100011
imm[11]    c : ........................0.......
imm[4:1]   a : ....................0100........
funct3       : .................001............ (bne)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
imm[10:5]  b : .000000.........................
imm[12]    d : 0...............................
imm label    : 0000000001000................... 0x8
part imm     : dcbbbbbbaaaa0...................
```
 
> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00100213 # PC=0   # addi x4, x0, 1  # accumulate
> 0x00300093 # PC=4   # addi x1, x0, 3   
> 0x00500113 # PC=8   # addi x2, x0, 5  
> 0x00209463 # PC=12  # bne x1, x2, 8   # JUMP START прыгаем на addr PC 12+8=20 (0xC+0x8=0x14 hex) 
> 0x00220213 # PC=16  # addi x4, x4, 2   
> 0x00120213 # PC=20  # addi x4, x4, 1  # JUMP TARGET
> ```
> Результат в:
> * x4=0x2 (1+1=2 результат не 1+2+1=4 так как инструкцию 0x00220213 мы перепрыгнули)
  
---

</details>


<br>
<details>
<summary> <b> # 35 Instruction: blt </b> </summary>

Инструкция `blt` (**B**ranch if **L**ess **T**han) — перейти, если меньше.

Она сравнивает значения в `rs1` и `rs2` как знаковые (signed) 32-битные числа. Если `rs1` строго меньше `rs2`, процессор совершает относительный прыжок на указанный сдвиг (offset) относительно текущего адреса PC.

```
blt rs1, rs2, offset
---
blt x1, x2, 8
```

| opcode | rs1 (с чем сравниваем) | rs2 (с чем сравниваем)| imm (константа) offset               |
| ---    | ---                    | ---                   | ---                                  |
| 7 бит  | 5 бит                  | 5 бит                 | 1+1+6+4=12 бит и +1 бит сами добавим |
| blt    | x1                     | x2                    | 8                                    |

 

```
PC = if (rs1 < rs2, signed) {PC + imm} else {PC + 4};
```

```
Instr hex    : 0x0020c463
Assembly     : blt x1, x2, 0x8
Format       : B
instr        : 00000000001000001100010001100011
opcode       : .........................1100011
imm[11]    c : ........................0.......
imm[4:1]   a : ....................0100........
funct3       : .................100............ (blt)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
imm[10:5]  b : .000000.........................
imm[12]    d : 0...............................
imm label    : 0000000001000................... 0x8
part imm     : dcbbbbbbaaaa0...................
```


> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00100213 # PC=0   # addi x4, x0, 1  # accumulate
> 0xFFF00093 # PC=4   # addi x1, x0, -1 # 0xFFFFFFFF 
> 0x00200113 # PC=8   # addi x2, x0, 2   
> 0x0020C463 # PC=12  # blt x1, x2, 8   # JUMP START, прыгаем на addr  PC 12+8=20 (0xC+0x8=0x14 hex)
> 0x00220213 # PC=16  # addi x4, x4, 2   
> 0x00120213 # PC=20  # addi x4, x4, 1  # JUMP TARGET
> ```
> Результат в:
> * x4=0x2 (1+1=2 результат не 1+2+1=4 так как инструкцию 0x00220213 мы перепрыгнули)
  
 
---

</details>


<br>
<details>
<summary> <b> # 36 Instruction: bltu </b> </summary>

Инструкция `bltu` (**B**ranch if **L**ess **T**han, **U**nsigned) - перейти, если меньше, без знака.

Она сравнивает значения в `rs1` и `rs2` как беззнаковые (unsigned) 32-битные числа. Если `rs1` строго меньше `rs2`, процессор совершает относительный прыжок на указанный сдвиг (offset) относительно текущего адреса PC.

```
bltu rs1, rs2, offset
---
bltu x1, x2, 8
```

| opcode | rs1 (с чем сравниваем) | rs2 (с чем сравниваем)| imm (константа) offset               |
| ---    | ---                    | ---                   | ---                                  |
| 7 бит  | 5 бит                  | 5 бит                 | 1+1+6+4=12 бит и +1 бит сами добавим |
| bltu   | x1                     | x2                    | 8                                    |


```
PC = if (rs1 < rs2, unsigned) {PC + imm} else {PC + 4};
```

```
Instr hex    : 0x0020e463
Assembly     : bltu x1, x2, 0x8
Format       : B
instr        : 00000000001000001110010001100011
opcode       : .........................1100011
imm[11]    c : ........................0.......
imm[4:1]   a : ....................0100........
funct3       : .................110............ (bltu)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
imm[10:5]  b : .000000.........................
imm[12]    d : 0...............................
imm label    : 0000000001000................... 0x8
part imm     : dcbbbbbbaaaa0...................
```


> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00100213 # PC=0   # addi x4, x0, 1  # accumulate
> 0x00100093 # PC=4   # addi x1, x0, 1   
> 0x00200113 # PC=8   # addi x2, x0, 2   
> 0x0020e463 # PC=12  # bltu x1, x2, 8  # JUMP START, прыгаем на addr  PC 12+8=20 (0xC+0x8=0x14 hex)
> 0x00220213 # PC=16  # addi x4, x4, 2   
> 0x00120213 # PC=20  # addi x4, x4, 1  # JUMP TARGET
> ```
> Результат в:
> * x4=0x2 (1+1=2 результат не 1+2+1=4 так как инструкцию 0x00220213 мы перепрыгнули)
  
 


---

</details>

<br>
<details>
<summary> <b> # 37 Instruction: bge </b> </summary>

Инструкция `bge` (**B**ranch if **G**reater than or **E**qual) — если `rs1` математически больше или равно `rs2`, процессор совершает относительный прыжок на указанный сдвиг (offset) относительно текущего адреса PC.

Она сравнивает значения в `rs1` и `rs2` как знаковые (signed) 32-битные числа. 

```
bge rs1, rs2, offset
---
bge x1, x2, 8
```

| opcode | rs1 (с чем сравниваем) | rs2 (с чем сравниваем)| imm (константа) offset               |
| ---    | ---                    | ---                   | ---                                  |
| 7 бит  | 5 бит                  | 5 бит                 | 1+1+6+4=12 бит и +1 бит сами добавим |
| bge    | x1                     | x2                    | 8                                    |



```
PC = if (rs1 >= rs2, signed) {PC + imm} else {PC + 4};
```

```
Instr hex    : 0x0020d463
Assembly     : bge x1, x2, 0x8
Format       : B
instr        : 00000000001000001101010001100011
opcode       : .........................1100011
imm[11]    c : ........................0.......
imm[4:1]   a : ....................0100........
funct3       : .................101............ (bge)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
imm[10:5]  b : .000000.........................
imm[12]    d : 0...............................
imm label    : 0000000001000................... 0x8
part imm     : dcbbbbbbaaaa0...................
```


> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00100213 # PC=0   # addi x4, x0, 1  # accumulate
> 0xFFF00093 # PC=4   # addi x1, x0, -1 # 0xFFFFFFFF 
> 0xfff00113 # PC=8   # addi x2, x0, -1   
> 0x0020d463 # PC=12  # bge x1, x2, 8   # JUMP START, прыгаем на addr  PC 12+8=20 (0xC+0x8=0x14 hex)
> 0x00220213 # PC=16  # addi x4, x4, 2   
> 0x00120213 # PC=20  # addi x4, x4, 1  # JUMP TARGET
> ```
> Результат в:
> * x4=0x2 (1+1=2 результат не 1+2+1=4 так как инструкцию 0x00220213 мы перепрыгнули)
  

---

</details>

<br>
<details>
<summary> <b> # 38 Instruction: bgeu </b> </summary>

Инструкция `bgeu` (**B**ranch if **G**reater than or **E**qual, **U**nsigned) — если `rs1` математически больше или равно `rs2`, процессор совершает относительный прыжок на указанный сдвиг (offset) относительно текущего адреса PC.

Она сравнивает значения в `rs1` и `rs2` как беззнаковые (unsigned) 32-битные числа. 

```
bgeu rs1, rs2, offset
---
bgeu x1, x2, 8
```

| opcode | rs1 (с чем сравниваем) | rs2 (с чем сравниваем)| imm (константа) offset               |
| ---    | ---                    | ---                   | ---                                  |
| 7 бит  | 5 бит                  | 5 бит                 | 1+1+6+4=12 бит и +1 бит сами добавим |
| bgeu   | x1                     | x2                    | 8                                    |


```
PC = if (rs1 >= rs2, siunsignedgned) {PC + imm} else {PC + 4};
```

```
Instr hex    : 0x0020f463
Assembly     : bgeu x1, x2, 0x8
Format       : B
instr        : 00000000001000001111010001100011
opcode       : .........................1100011
imm[11]    c : ........................0.......
imm[4:1]   a : ....................0100........
funct3       : .................111............ (bgeu)
rs1 (source) : ............00001............... x1 ("ra")
rs2 (source) : .......00010.................... x2 ("sp")
imm[10:5]  b : .000000.........................
imm[12]    d : 0...............................
imm label    : 0000000001000................... 0x8
part imm     : dcbbbbbbaaaa0...................
```



> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x00100213 # PC=0   # addi x4, x0, 1  # accumulate
> 0x00100093 # PC=4   # addi x1, x0, 1  
> 0x00100113 # PC=8   # addi x2, x0, 1   
> 0x0020f463 # PC=12  # bgeu x1, x2, 8  # JUMP START, прыгаем на addr  PC 12+8=20 (0xC+0x8=0x14 hex)
> 0x00220213 # PC=16  # addi x4, x4, 2   
> 0x00120213 # PC=20  # addi x4, x4, 1  # JUMP TARGET
> ```
> Результат в:
> * x4=0x2 (1+1=2 результат не 1+2+1=4 так как инструкцию 0x00220213 мы перепрыгнули)
  
  

</details>

---

## Инструкции U-типа 
(Upper Immediate)

*Применяется для загрузки больших констант в старшие 20 бит регистра (инструкции `lui` и `auipc`). Поля `rs1` и `funct3` здесь отсутствуют.*

Способ извлечения константы (Immediate) из кода инструкции: `imm[31:12]` (20 бит, сдвигается влево на 12)

| imm[31:12] | rd (приёмник) | opcode (код операции) |
| ---        | ---           | ---                   |
| **31:12**  | **11:7**      | **6:0**               |
| 20 бит     | 5 бит         | 7 бит                 |


<br>
<details>
<summary> <b> # 39 Instruction: lui </b> </summary>

Инструкция `lui` (Load Upper Immediate) - загрузка непосредственного значения в старшие биты.

Все инструкции RISC-V имеют фиксированный размер — 32 бита. Из-за этого вы физически не можете за один раз положить 32-битное число в регистр, ведь часть битов самой инструкции обязательно уходит на кодирование её имени (opcode) и номера регистра назначения (rd).

В инструкции `lui` биты `imm[31:12]` помещаются в старшие биты регистра `rd`, а младшие 12 бит регистра `rd` заполняются нулями для получения 32-х битного числа.

Вы хотите выполнить команду `lui x10, 0x12345`. Результат в регистре `x10=0x12345000`, сама по себе `lui` записывает только старшую часть числа, оставляя в конце нули. Для получения точного 32-битного числа, например `0x12345678`, применяется дополнительно инструкция `addi`. Добавим к числу `0x12345000` в младшие биты число `0x678`, что бы получить точное число `0x12345678` с помощью иснтрукции `addi x10, x10, 0x678`:

```
lui rd, imm
---
lui  x10, 0x12345      # x10 теперь равен 0x12345000
addi x10, x10, 0x678   # x10 = 0x12345000 + 0x678 = 0x12345678
```
 
| opcode | rd (регистр-приёмник данных)| imm (константа) |
| ---    | ---                         | ---             |
| 7 бит  | 5 бит                       | 20 бит          |
| lui    | x10                         | 0x12345         |

```
hexadecimal  : 0x12345537
Assembly     : lui x10, 0x12345000
Format       : U
instr        : 00010010001101000101010100110111
opcode       : .........................0110111 (lui)
rd (receiver): ....................01010....... x10 ("a0")
imm          : 00010010001101000101............ raw=0x12345000
```


> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x0
> 0x12345537   # lui x10, 0x12345000 # imm 20 bit
> 0x67850593   # addi x11, x10, 0x678
> ```
> Результат в:
> * x10=0x12345000  
> * x11=0x12345678
 

---

</details>

<br>
<details>
<summary> <b> # 40 Instruction: auipc </b> </summary>

Инструкция `auipc` (Add Upper Immediate to PC) - прибавить непосредственное значение в старшие биты к счетчику команд PC.
 
Это критически важная инструкция для создания позиционно-независимого кода (PIC). Она позволяет процессору узнать, где в памяти он сейчас находится `auipc x1, 0x0`, и вычислить абсолютный адрес глобальной переменной или функции, даже если всю программу переместили в другой адресный регистр.

Точно так же, как `lui` работает в паре с `addi` для создания 32-битных констант, инструкция `auipc` работает в паре с `jalr` (или инструкциями `lw/sw`) для осуществления дальних переходов или обращений к памяти в радиусе $\pm2$ ГБ от текущего положения кода.

В инструкции `auipc` биты `imm[31:12]` помещаются в старшие биты регистра `rd`, а младшие 12 бит регистра `rd` заполняются нулями для получения 32-х битного числа. Это сформированное число складывается с текущим значением регистра PC (адресом, по которому лежит сама эта команда auipc) и записывается в целевой регистр `rd`.


```
auipc rd, imm
---
auipc x10, 0x00020
```

Значение из imm `0x00020` после преобразования в старшие 20 бит 32-битного значения станет `0x00020000`

| opcode  | rd (регистр-приёмник данных)| imm (константа) |
| ---     | ---                         | ---             |
| 7 бит   | 5 бит                       | 20 бит          |
| auipc   | x10                         | 0x00020         |


> [!EXAMPLE]
> Инструкция auipc расположена в ROM по адресу 0x8 т.е. PC=8
>
> ROM:
> ```
> v2.0 raw
> 0x0        # PC=0
> 0x0        # PC=4
> 0x00020517 # PC=8 # auipc x10, 0x00020 # (0x00020 => 0x00020000)
>     
> ```
> Результат в:
> * x10=0x00020008 (0x00020000+0x8=0x00020008)  
 
 

</details>

---

## Инструкции J-типа 
(Jump / Безусловный переход)

*Применяется для инструкции прыжка с сохранением адреса возврата (`jal`). Содержит 20-битную константу смещения. Как и в B-типе, адрес всегда кратен 2 байтам (бит `imm[0]` опущен), а биты константы сильно перемешаны для минимизации логических гейтов на кристалле процессора.*

 
<br>
<details>
<summary> <b> # 41 Instruction: jal </b> </summary>

Инструкция `jal` (**J**ump **a**nd **L**ink) — прямой относительный безусловный переход и связывание. Передача управления по адресу со смещением относительно PC (PC-relative), заданного 20-битным непосредственным значением со зна­ком, и сохранение адреса следующей инструкции (адрес возврата) в регистре-приемнике.

> `jal` это относительный (прямой) безусловный переход, что означает адрес для прыжка жестко «привязан» к месту, где написана сама инструкция т.е. относительно текущего вызова команды, а [`jalr`](#Группа-jalr) это абсолютный (косвенный) безусловный переход, что означает адрес для прыжка не зависит от места вызова инструкции, а расчитывается взятием адреса из регистра плюс смещение `imm`. 

Способ извлечения константы (Immediate) из кода инструкции непоследовательный. 
Старшие 20 бит собираем из инструкции, в такой последовательности - `imm[20] + imm[19:12] + imm[11] + imm[10:1]` и сами дописываем в младшую позицию 0 (так как адрес всегда кратен 2), теперь у нас полный набор imm из 21 бита.

Диапазон прыжка: $\pm 1$ МБ от текущей инструкции. Если функция находится дальше, мы сначала используем `auipc` (загружаем старшую часть адреса в регистр), а затем прыгаем через `jalr`.

| imm[20]   | imm[10:1] | imm[11]   | imm[19:12] | rd (куда сохранить адрес возврата) | opcode  |
| ---       | ---       | ---       | ---        | ---                                | ---     |
| **31**    | **30:21** | **20**    | **19:12**  | **11:7**                           | **6:0** |
| 1 бит     | 10 бит    | 1 бит     | 8 бит      | 5 бит                              | 7 бит   |
| 4-я часть | 1-я часть | 2-я часть | 3-я часть  |                                    |         |


```
inst[31:12]: 01100100001100101010
imm:  [20] [19:12]   [11] [10:1]    0
      0    001010101 1    100100001 0
```

Текущий PC складывается с imm. Это и есть новый адрес: $\text{New PC} = PC + imm$

В регистр `rd` записывается адрес возврата: `rd = Current PC + 4`

На следующем такте PC обновляется значением $\text{New PC}$


```
jal rd, imm (label)
---
jal x1, 0x8

# Выполняет две операции одновременно:
# x1 = PC + 4
# PC = PC + 8
```

| opcode | rd (куда сохранить адрес возврата) | imm (константа) label                 |
| ---    | ---                                | ---                                   |
| 7 бит  | 5 бит                              | 1+10+1+8=20 бит и +1 бит сами добавим |
| jal    | x1                                 | 0x8                                   |



```
Instr hex    : 0x008000ef
Assembly     : jal x1, 0x8
Format       : J
instr        : 00000000100000000000000011101111
opcode       : .........................1101111
rd (receiver): ....................00001....... x1 ("ra")
imm[19:12] c : ............00000000............
imm[11]    b : ...........0....................
imm[10:1]  a : .0000000100.....................
imm[20]    d : 0...............................
imm label    : 000000000000000001000............ 0x8
part imm     : dccccccccbaaaaaaaaaa0............
```

> [!EXAMPLE]
>
> ROM:
> ```
> v2.0 raw
> 0x0        # PC=0
> 0x0        # PC=4
> 0x00300193 # PC=8     # addi x3, x0, 3 # accumulate
> 0x008000ef # PC=0xC   # jal x1, 0x8    # JUMP START, addr PC 0xC+0x8=0x14
> 0x00418193 # PC=0x10  # addi x3, x3, 4
> 0x00218193 # PC=0x14  # addi x3, x3, 2 # JUMP TARGET
> ```
> Результат в:
> * x1=0x10 (0xC + 4 = 0x10, адрес возврата - это следующая инструкция после адреса инструкции с прыжком)
> * x3=0x5 (3+2=5 результат не 3+4+2=9 так как инструкцию 0x00418193 мы перепрыгнули)
 
  
</details>

  
---

## Zicsr расширение Control and Status Registers (CSR)

Расшифровка:
* Z — стандартное расширение (не обязательное).
* icsr — Integer Control and Status Registers.
  
У процессора есть скрытая от обычных программ жизнь: прерывания, счётчики времени, режимы защиты, обработка ошибок. Для управления всем этим и созданы CSR.

(инструкция `ecall`, `ebreak`, `mret`, режимы привилегий, trap, прерывания - относятся к `Privileged ISA` )

CSR-инструкции используют модель привилегированных регистров (`mstatus`, `mtvec`, `mepc` и т.д.).

Обработчики исключений при выполнении своей работы используют четыре специальных регистра, называемых регистрами управления и состояния (control and state register, CSR): mtvec, mcause, mepc и mscratch. Регистр базового адреса вектора ловушек, mtvec, содержит адрес обработчика исключений. Когда возникает исключение, процессор записывает причину исключения в mcause, сохраняет в mepc значение счетчика команд для инструкции, которая вызвала исключение, и переходит к обработчику исключения по адресу, предварительно указанному в mtvec.

Перейдя по адресу в mtvec, обработчик исключений читает регистр mcause, чтобы выяснить, что вызвало исключение, и реагирует соответствующим образом (например, считывая код нажатой клавиши при аппаратном прерывании). Затем он либо прерывает выполнение программы, либо возвращается в программу, выполняя mret, инструкцию возврата из машинного исключения, которая переходит к адресу, сохраненному в mepc. Сохранение в mepc адреса инструкции, которая вызвала исключение, аналогично использованию регистра ra для сохранения адреса возврата во время выполнения инструкции jal. Обработчики исключений используют программные регистры (x1-x31), поэтому они применяют область памяти, на которую указывает mscratch, для сохранения и восстановления этих регистров.
 
> Режимы выполнения и уровни привилегий
> 
> Машинный режим (M-режим) – это наивысший уровень привилегий; программа, работающая в этом режиме, может получить доступ ко всем регистрам и ячейкам памяти. M‐режим – это единственный режим привилегий, используемый в процессорах, работающих без операционной системы (ОС), включая многие встраиваемые системы.
> Пользовательские приложения, которые работают поверх ОС, обычно работают в пользовательском режиме (U-режим), а ОС работает в режиме супервизора (S режим). Пользователь ские программы не имеют доступа к привилегированным регистрам или ячейкам памяти, зарезервированным для ОС. В этом и заключается смысл использования разных режимов – они защищают состояние системы от повреждения.

Перечень регистров, связанных с исключениями, зависит от режима работы. Регистры M‐режима – это mtvec, mepc, mcause и mscratch, а регистры S‐режима – sepc, scause и sscratch. Для H‐режима также есть свои регистры. Отдельные регистры исключений, выделенные для каждого режима, обеспечивают аппаратную поддержку нескольких уровней привилегий.

| Команда | funct3 | Описание                                                  | Что пишется обратно в CSR-регистр       |
| ------- | ------ | ---                                                       | ---                     |
| csrrw   | `001`  | Atomic Read/Write CSR                                     | Data т.е. RegFile[rs1]  |
| csrrs   | `010`  | Atomic Read and Set Bits                                  | CSRReadData (т.е. старое значение CSR ) |
| csrrc   | `011`  | Atomic Read/Clear Bits                                    | `CSRReadData & (~Data)` |
| csrrwi  | `101`  | вместо регистра `rs1` использует 5-битное число Immediate | Data т.е. imm           |
| csrrsi  | `110`  | вместо регистра `rs1` использует 5-битное число Immediate | CSRReadData             |
| csrrci  | `111`  | вместо регистра `rs1` использует 5-битное число Immediate | `CSRReadData & (~Data)` |
 
Для работы с системными регистрами используются псевдоинструкции, которые упрощают чтение и запись.

| Псевдоинструкция | Базовая инструкция  | Описание                                |
| ---              | ---                 | ---                                     |
| `csrr rd, csr`   | `csrrs rd, csr, x0` | Чтение значения CSR в регистр `rd`.     |
| `csrw csr, rs`   | `csrrw x0, csr, rs` | Запись значения из `rs` в CSR.          |
| `csrs csr, rs`   | `csrrs x0, csr, rs` | Установка битов в CSR по маске из `rs`. |
| `csrc csr, rs`   | `csrrc x0, csr, rs` | Сброс битов в CSR по маске из `rs`.     |


Все 6 инструкций делают по сути одно базовое действие: они одновременно читают старое значение из CSR и записывают туда новое. 
Разница между ними заключается только в способе формирования нового значения, которое улетит в CSR-регистр. Старое значение всегда выдаётся на шину CSRReadData в неизменном виде.


**Инструкция `mret`** возврат из обработчика прерываний, которая аппаратно использует 
адрес возврата из регистра `mepc` и восстанавливает глобальное разрешение к перываниям  

```
MIE  <- MPIE
MPIE <- 1
PC   <- mepc
```
 

| Команда | funct3  | imm             | Код инструкции | Описание                                                   |
| ------- | ------  | ---             | ---            | ---                                                        |
| `mret`  | `0b000` | `001100000010`  | `0x30200073`   | (**M**achine **Ret**urn) возврат из обработчика прерываний |


```
31            20 19   15 14  12 11    7 6      0
+---------------+-------+------+-------+--------+
| csr[11:0]     | rs1   |funct3|  rd   |1110011 |
+---------------+-------+------+-------+--------+
```


#### Инструкция `csrrw` (Atomic **R**ead/**W**rite CSR) - полная перезапись. 

Атомарно читает значение из CSR в регистр общего назначения `rd` и одновременно записывает значение из `rs1` в этот же CSR.

Инструкция передает адрес регистра (12-битное поле `imm[11:0]` из инструкции) в блок CSR, вычитывает старое значение на мультиплексор записи в регистровый файл (RegWrite), а новое значение из шины `rs1` записывает в выбранный CSR.

Пример, прочитать `mtvec` в `x5`, и записать `x6` в `mtvec`:
```
csrrw x5, mtvec, x6 
```


#### Инструкция `csrrs` (Atomic **R**ead and **S**et Bits) - установка конкретных битов в 1 (битовая маска). 

Используется для чтения системных регистров без их изменения (если `rs1 = x0`)

Процессор берёт значение из rs1. Там, где в rs1 стоят единицы, соответствующие биты внутри CSR-регистра включаются в 1. Остальные биты CSR остаются нетронутыми. Логически это операция CSR = CSR | rs1.

Если на входе `rs1` передается `x0`, запись в CSR блокируется, а текущее значение выбранного CSR просто уходит в регистр `rd`.

```
csrr x5, mcause # псевдокод для csrrs x5, mcause, x0
```


#### Инструкция `csrrc` (Atomic **R**ead/**C**lear Bits) - сброс конкретных битов в 0 (битовая маска). 

Процессор берёт значение из `rs1`. Там, где в `rs1` стоят единицы, соответствующие биты внутри CSR-регистра выключаются в 0. Логически это операция `CSR = CSR & (~rs1)`.



---

<br>
<details>
<summary> <b> Модуль: CSR Unit </b> </summary>

<br>
 
| CSR        | bin            | hex   | Назначение                                  |
| ---------- | ---            | ---   | -------------------------                   |
| `mstatus`  | `001100000000` | 0x300 | состояние процессора                        |
| `mie`      | `001100000100` | 0x304 | разрешение прерываний                       |
| `mtvec`    | `001100000101` | 0x305 | адрес обработчика прерываний ОС (trap)      |
| `mscratch` | `001101000000` | 0x340 | ОС хранит указатель на свой стек, чтобы в самый первый момент поменять его местами с пользовательским вектором.  |
| `mepc`     | `001101000001` | 0x341 | адрес возврата в программу                  |
| `mcause`   | `001101000010` | 0x342 | причина исключения trap (для ECALL туда пишется 8) |
| `mtval`    | `001101000011` | 0x343 | дополнительная информация                   |
| `mip`      | `001101000100` | 0x344 | ожидающие прерывания                        |
| `cycle`    | `110000000000` | 0xC00 | счётчик тактов процессора, младшие 32 бита  |
| `cycleh`   | `110010000000` | 0xC80 | счётчик тактов процессора, старшие 32 бита  |
| `time`     | `110000000001` | 0xC01 | счётчик времени, младшие 32 бита            |
| `timeh`    | `110010000001` | 0xC81 | счётчик времени, старшие 32 бита            |


Железо процессора обязано сохранить в `mepc` точный адрес вызвавшей его команды, поэтому сохраним чистый адрес самой инструкции `ecall`.

Это сделано специально: операционной системе (ОС) жизненно необходимо знать, какая конкретно строчка кода или какая системная функция (через регистры) её вызвала. Если бы железо автоматически сохраняло `PC + 4`, ОС никогда бы не узнала точный адрес вызова (есть еще сжатые команды, у них адреса с инкрементом +2 ).

Поскольку в полноценной системе обработчик прерываний — это не одна строчка `mret`, а целый кусок кода в памяти, перед самым выходом операционная система делает следующее:
* Считывает значение из `mepc` в обычный регистр (например, `x6`) с помощью инструкции чтения CSR.
* Прибавляет к этому регистру 4.
* Записывает обновленное значение обратно в `mepc` с помощью инструкции записи CSR.
* Выполняет `mret`.


<br>
<details>
<summary> <b> Регистры cycle и cycleh </b> </summary>
 
Каждый тактовый импульс значение внутри `cycle` увеличивается на 1.

Когда `cycle` переполняется (доходит до `0xFFFFFFFF`), сигнал переноса инкрементирует регистр `cycleh`.

Программа не может записать туда такты — она может только считать их оттуда инструкцией вроде `csrrw x5, cycle, x0`, чтобы замерить скорость выполнения кода или настроить задержку.

* Работа  по такому каскадному принципу, сквозного составного 64-битного счётчика:
  
  Младшая половина (`cycle`) непрерывно считает каждый такт от 0 до `0xFFFFFFFF`.

  Как только в `cycle` накопилось максимальное значение `0xFFFFFFFF`, на следующем тактовом импульсе он сбрасывается в `0x00000000`. В этот же самый момент его флаг переноса `c_o` (Carry Out) подталкивает регистр `cycleh`, и тот делает шаг на +1.

  Пока cycle бежит свой следующий круг от 0 до максимума, `cycleh` статично хранит свою единицу.

  При новом переполнении младшего регистра `cycleh` станет равен 2, затем 3 и так далее.

  Когда-нибудь (правда, при частоте процессора в несколько мегагерц на это уйдут сотни лет) оба регистра заполнятся единицами до упора — и в `cycleh`, и в `cycle` будет лежать `0xFFFFFFFF`. На следующем после этого такте вся система физически обнулится, и счёт начнётся с чистого листа: `0x00000000_00000000`

</details>
<br>


 
<details>
<summary> <b> Регистры time и timeh </b> </summary>
 
Это регистры для счета времени.
 
По спецификации RISC-V `time` обычно увеличивается от внешнего таймера с фиксированной частотой (например, 1 МГц), независимо от частоты ядра. 

Т.е. регистр `cycle` считает такты CPU, а регистр `time` считает тики таймера.


При `csrr x5, time`, `csrr x5, timeh`  модуль `CSRUnit` обрабатывает адреса для чтения регистров `time` и `timeh` которые просто берут свои значения из модуля `TimerUnit` от регистров `mtime_lo` и `mtime_hi`. Т.е. регистры `time` и `timeh` — это виртуальные CSR, у них нет своих регистров внутри `CSRUnit`, они являются "окном" к регистрам mtime_lo и mtime_hi из `TimerUnit`.


### Модуль Timer Unit: mtime и mtimeh

`mtime` — системный таймер, который используется для генерации таймерных прерываний.

`mtimecmp` — регистр сравнения, содержит величину при превышении которой (`time >= mtimecmp`) произойдет сигнал к прерыванию Timer Interrupt. В реальных системах он находится не в CSR, а по адресу памяти, так как это **memory-mapped register**, ему нужен свой адрес в пространстве MMIO:
```
# управление условием прерывания
0x10000004	mtimecmp_lo
0x10000008	mtimecmp_hi

# При записи по адресу 0x10000004 или 0x10000008 происходит установка нового значения в регистр mtimecmp:
 174:   10000337                lui     t1,0x10000
 178:   00430313                addi    t1,t1,4 # 10000004 <__data_load+0xffffe3c>
 17c:   00732023                sw      t2,0(t1)
```


```
Регистры CSR time и timeh просто возвращают младшие и старшие 32 биты регистра mtime

CSR time  ----\
               >--- читают mtime
CSR timeh ----/

if (mtime_hi > mtimecmp_hi)
    timer_irq = 1
else if (mtime_hi == mtimecmp_hi &&
         mtime_lo >= mtimecmp_lo)
    timer_irq = 1
else
    timer_irq = 0
```
 


**Тест**

**Файл test_trap_time.S**
```
.section .text
.global main

main:
 
    # START устанавливаем время 30 до следующего прерывания Timer Interrupt
    lui  x5, 0x10000      # x5 = 0x10000000
    addi x5, x5, 4        # x5 = 0x10000004
    addi x6, x0, 30
    sw x6, 0(x5)

sub_loop:
    # Печатаем 'A'
    li t0, 'A'
    li t1, 0x10000000
    sb t0, 0(t1)
    j sub_loop

loop:
    j main

```

Компиляция:
```
riscv64-unknown-elf-gcc -march=rv32i_zicsr -mabi=ilp32 -nostdlib -T linker.ld startup.S trap.S test_trap_time.S -o test_trap_time.elf

riscv64-unknown-elf-objcopy -O binary test_trap_time.elf test_trap_time.bin
```

Результат: каждые 1000 тиков происход событие Timer Interrupt 

</details>
<br>


 
</details>


---


В RISC-V, исключения (exception) и внешние прерывания (interrupt) отличаются только причиной возникновения (`mcause`) но называются одинаково - **trap**, так как имеют одинаковую последовательность действий.

```
Trap
↓
сохранить PC в mepc
↓
записать причину в mcause
↓
обновить mstatus разрешение на прерывания
↓
основываясь на причине, перейти по адресу из mtvec в обработчик  
↓
выполнить обработчик
↓
завершить работу обработчика через вызов mret, восстановить mstatus
↓
вернуться обратно в программу на следующую инструкцию
```

<br>
<details>

<summary> Реализованные исключения и прерывания</summary>

* **ecall** (cause[31]=0 mcause=11 in Machine mode) — программное исключение (System Call / Syscall)

* **ebreak** (cause[31]=0 mcause=3) — программное исключение для отладки

* **Store/AMO Access Fault** (cause[31]=0 mcause=7) — если процессор пытается записать и вычисленный адрес (база + смещение) указывает в «пустоту»

* **Load Access Fault** (cause[31]=0 mcause=5) — если процессор пытается прочитать и вычисленный адрес (база + смещение) указывает в «пустоту»


* **Instruction Access Fault** (cause[31]=0 mcause=1) — если счетчик команд pc указывает на адрес за пределами допустимого диапазона

* **IllegalInstruction**  (cause[31]=0 mcause=2) — неподдерживаемая инструкция, неизвестный opcode 

* **Machine External Interrupt** (cause[31]=1 mcause=0x8000000B) — внешнее аппаратное прерывание от периферии или внешнего контроллера прерываний (UART, GPIO, сетевой контроллер и т.п.).

* **Machine Software Interrupt** (cause[31]=1 mcause=0x80000003) — программное прерывание, обычно инициируемое другим ядром процессора или системным программным обеспечением

* **Machine Timer Interrupt** (cause[31]=1 mcause=0x80000007) — аппаратное прерывание таймера, возникающее, когда выполняется условие mtime >= mtimecmp

</details>




**Формирование причины (mcause):**

Если причина для trap это исключение то 31-й бит 0, если это прерывание то 31-й бит 1.

* Исключение вызванные програмно при `ecall`, `ebreak` в Машинном режиме (M) формируется так: cause[31]=0 cause[0:30]=11 в итоге в регистр mcause пойдет 11.

* Исключение вызванные аппаратно по различных исключительным ситуациям требующим вмешательства. Биты причины для `mcause`: cause[31]=0 mcause[0:30]=... Например для `IllegalInstruction`: cause[31]=0 mcause[0:30]=2 в итоге в регистр `mcause` пойдет 2.

* Внешнее прерывание (interrupt) irq — это аппаратный сигнал, приходящий извне процессора, т.е. просто входной провод процессора. Биты причины для `mcause` вычисляются так: `mcause = (interrupt << 31) | cause` т.е. cause[31]=1 cause[0:30]=...  Например для `ExternalInterrupt` в mcause пойдет `0x8000000B` 

> [!WARNING]
> Для снятие флага ожидания прерывания `mip.MEIP` или `mip.MSIP` используют не инструкции, а выделяют адрес в MMIO (memory-mapped register), сигнал записи по этому адресу будет значить сброс флага, так как только в обработчике есть информация когда он заршил работу.
> А для флага `mip.MTIP` снятие происходит аппаратно при условии `mtime < mtimecmp` но новое значение для mtimecmp устанавливается программно в обработчике

```
# управление условием прерывания (снять нажатую кнопку)
0x10000010	MEIP_CLEAR
0x10000018	MSIP_CLEAR

---
    # Снять MEIP
    li t0,0
    li t1,0x10000010
    sw t0,0(t1)    
---
    # Снять MSIP
    li t0,0
    li t1,0x10000018
    sw t0,0(t1)
```

Каждый источник прерывания имеет свой бит в регистре mip:

```
 31                                 12  11   10  9    8   7    6   5    4   3    2   1    0
+-------------------------------------+----+---+----+---+----+---+----+---+----+---+----+---+
|                WPRI                 |MEIP| 0 |SEIP| 0 |MTIP| 0 |STIP| 0 |MSIP| 0 |SSIP| 0 |
+-------------------------------------+----+---+----+---+----+---+----+---+----+---+----+---+

бит 7  MTIP  (Machine Timer Interrupt Pending)
бит 11 MEIP  (Machine External Interrupt Pending) GPIO/UART
бит 3  MSIP  (Machine Software Interrupt Pending) Software

```


```
31  30                  0
+---+-------------------+
| I |      Cause        |
+---+-------------------+

бит 31 (I) = 0 → исключение (Exception)
бит 31 (I) = 1 → прерывание (Interrupt)
биты 30:0 = код причины
```

**Формирование статуса**. Регистр mstatus (Machine Status Register)

 
* Бит 3 (MIE) — (Machine Interrupt Enable) глобальное разрешение прерываний в M-режиме (1 = включены, 0 = выключены)
* Бит 7 (MPIE) — (Machine Previous Interrupt Enable) копия MIE до исключения
* Биты 12:11 (MPP) — режим, в котором процессор работал до исключения:
  * 0b00 = U-режим (User)
  * 0b01 = S-режим (Supervisor) — если есть
  * 0b11 = M-режим (Machine)

Без других уровней привилегий, MPP нигде не участвует в принятии решений. Он становится нужен только тогда, когда появятся User/Supervisor Mode.


 

Подготовка к обработке trap:
* проверка разрешения для trap `mstatus.MIE == 1` (только для разрешения внешнего IRQ)
* mstatus.MPP = текущий_режим;  // например, если вы были в U-режиме, MPP = 0b00
* mstatus.MPIE ← mstatus.MIE
* mstatus.MIE = 0;              // отключил прерывания (глобально)

Выход из обработчика trap:
* mstatus.MIE = mstatus.MPIE;   // Восстанавливаем старый MIE
* mstatus.MPIE = 1;             // Устанавливаем в 1 (запасное значение)
* mstatus.MPP = 0b00;           // Сбрасываем MPP в U-режим (или 0)
 
Проверка mstatus.MIE на разрешение создать прерывание: 
* Исключения (включая программные прерывания через ecall/ebreak, ошибки выравнивания, Illegal Instruction и т.д.) являются синхронными. Они привязаны к конкретной инструкции в потоке выполнения и им не нужна проверка, они немедленно начинают прерывание.
* Аппаратные прерывания (внешние IRQ, таймер, программные межпроцессорные прерывания) — это **асинхронные** события. Они приходят извне ядра и могут быть обработаны немедленно или отложены. Если процессор уже работает в M-mode, прерывания разрешены только если `mstatus.MIE = 1`. Но аппаратный сигнал от внешнего устройства не теряется, он фиксируется в регистре mip (Machine Interrupt Pending) в бите MEIP: `mip[11]=1` и если сейчас разрешены прерывания т.е. `mstatus.MIE == 1` то мы инициируем trap для IRQ и сбрасываем `mip[11]=0`, если нет т.е. `mstatus.MIE == 0`, то мы ждем разрешения и после его получения инициируем trap для IRQ и сбрасываем `mip[11]=0`

 

> [!IMPORTANT]
> Рабочие файлы процессора **RV32I + расширения Zicsr и M**: startup.S, trap.S и linker.ld

**Файл startup.S:**


```
.section .text
.global _start

.extern main

.extern __stack_top
.extern __data_load
.extern __data_start
.extern __data_end

.extern __bss_start
.extern __bss_end

_start:

    # 1. Инициализация стека
    la sp, __stack_top

    # 2. Установить адрес общего обработчика trap
    la t0, trap_handler
    csrw mtvec, t0

    # 3. Копирование .data из ROM в RAM
    # цикл: ROM to RAM, копирует все слова секции .data
    la t0, __data_load
    la t1, __data_start
    la t2, __data_end

copy_data:
    beq t1, t2, copy_done

    lw t3, 0(t0)
    sw t3, 0(t1)

    addi t0, t0, 4
    addi t1, t1, 4

    j copy_data

copy_done:

    # 3. Обнулить .bss
    la t1, __bss_start
    la t2, __bss_end

clear_bss:
    beq t1, t2, bss_done
    sw zero, 0(t1)
    addi t1, t1, 4
    j clear_bss

bss_done:

    # Разрешаем внешние прерывания в регистре mie (11-й бит)
    li t0, 0x800
    csrs mie, t0

    # 4. Разрешаем глобальные прерывания в mstatus (MIE=1)
    li t0, 0x8
    csrs mstatus, t0

    # 5. Вызвать main
    call main

    # Если main вернулся — зависнуть

hang:
    j hang

```


**Файл trap.S** (вся обработка исключений и прерываний)

```
.section .text
.global trap_handler

##################################################
# Общий обработчик trap (Direct Mode)
##################################################

trap_handler:

    # mcause
    csrr t0, mcause

    ##############################################
    # Interrupt
    ##############################################

    # External Interrupt
    li t1, 0x8000000B
    beq t0, t1, irq_handler

    # Software Interrupt
    li t1, 0x80000003
    beq t0, t1, software_handler


    # Timer Interrupt
    li t1, 0x80000007
    beq t0, t1, timer_handler

    ##############################################
    # Exception
    ##############################################

    li t1, 11
    beq t0, t1, ecall_handler

    li t1, 3
    beq t0, t1, ebreak_handler

    li t1, 2
    beq t0, t1, illegal_handler

    li t1, 1    # Instruction Access Fault
    beq t0, t1, memory_error

    li t1, 5    # Load Access Fault
    beq t0, t1, memory_error

    li t1, 7    # Store Access Fault
    beq t0, t1, memory_error

hang:
    j hang

##################################################
# Instruction Access Fault
##################################################

memory_error:

    li t0, 0x4D        # 77 'M'
    li t1, 0x10000000   
    sb t0, 0(t1)        

    # Просто зависаем здесь, чтобы симуляция не улетела в космос
    memory_hang:
        j memory_hang

##################################################
# ECALL
##################################################

ecall_handler:

    li t0,0x45 # 69 'E'
    li t1,0x10000000
    sb t0,0(t1)

    csrr t2,mepc
    addi t2,t2,4
    csrw mepc,t2

    mret

##################################################
# EBREAK
##################################################

ebreak_handler:

    li t0,0x4B # 75 'K'
    li t1,0x10000000
    sb t0,0(t1)

    csrr t2,mepc
    addi t2,t2,4
    csrw mepc,t2

    mret

##################################################
# Illegal Instruction
##################################################

illegal_handler:

    li t0,73 # 'I'
    li t1,0x10000000
    sb t0,0(t1)

    csrr t2,mepc
    addi t2,t2,4
    csrw mepc,t2

    mret

##################################################
# External Interrupt
##################################################

irq_handler:

    li t0,81 # 'Q'
    li t1,0x10000000
    sb t0,0(t1)

    # Снять MEIP
    li t0,0
    li t1,0x10000010
    sw t0,0(t1)

    mret

##################################################
# Timer Interrupt
##################################################

timer_handler:

    li t0,'T'
    li t1,0x10000000
    sb t0,0(t1)

    # Снятие MTIP аппаратно

    # Перенести следующее событие таймера
    # mtimecmp = mtime + период

    # читаем mtime_lo (mtime_lo)
    csrr t2, 0xC01

    # Следующее срабатывание через 1000 тактов
    li t3,1000
    add t2,t2,t3

    # Записать новое значение mtimecmp_lo
    li t1,0x10000004
    sw t2,0(t1)

    mret    

##################################################
# Software Interrupt
##################################################

software_handler:

    # Вывести 'S'
    li t0, 'S'
    li t1, 0x10000000
    sb t0, 0(t1)

    # Снять MSIP
    li t0,0
    li t1,0x10000018
    sw t0,0(t1)

    mret
```    


**Файл linker.ld**

```
/* linker.ld */
MEMORY
{
  rom (rx) : ORIGIN = 0x00000000, LENGTH = 2M
  ram (rwx): ORIGIN = 0x80000000, LENGTH = 8M
}

SECTIONS
{
  . = 0x00000000;

  .text :
  {
    *(.text*)
    *(.text.*)
  } > rom

  .rodata :
  {
    *(.rodata*)
    *(.rodata.*)
  } > rom

  __data_load = . ;     

  .data :
  {
    __data_start = . ;
    *(.data*)
    *(.sdata*)
    . = ALIGN(4);
    __data_end = . ;
  } > ram AT > rom

  .bss :
  {
    __bss_start = .;
    *(.bss*)
    *(.sbss*)
    . = ALIGN(4);
    __bss_end = .;
  } > ram

  __stack_top = ORIGIN(ram) + LENGTH(ram);
}
```


 
<br>
<details>
<summary><b>Тестирование CSR</b> </summary>
 

**Файл test_trap.S**
```
.section .text
.global main

main:
    
    # Печатаем 'A'
    li t0, 0x41 # 65 'A'
    li t1, 0x10000000
    sb t0, 0(t1)

    # Вызываем исключение
    ecall

    # После возврата из обработчика
    li t0, 'B'
    sb t0, 0(t1)
    
    # Illegal Instruction
    unimp 

    li t0, 'C'
    sb t0, 0(t1)

loop:
    j main
 
```    

**Компиляция test_trap.S**
```
riscv64-unknown-elf-gcc -march=rv32i_zicsr -mabi=ilp32 -nostdlib -T linker.ld startup.S trap.S test_trap.S -o test_trap.elf
```

**Создание чистого бинарного файла (flat binary)**
```
riscv64-unknown-elf-objcopy -O binary test_trap.elf test_trap.bin
```

**Создание hex-dump**  
```
(echo "v2.0 raw"; hexdump -v -e '1/4 "0x%08x\n"' test_trap.bin) > test_trap.hex
```
 
В ROM можно загрузить test_trap.bin или test_trap.hex.

**Дизассемблировать все секции с кодом**
```
riscv64-unknown-elf-objdump -D test_trap.elf

---

test_trap.elf:     file format elf32-littleriscv


Disassembly of section .text:

00000000 <_start>:
   0:   80800117                auipc   sp,0x80800
   4:   00010113                mv      sp,sp
   8:   00000297                auipc   t0,0x0
   c:   07428293                addi    t0,t0,116 # 7c <trap_handler>
  10:   30529073                csrw    mtvec,t0
  14:   1cc00293                li      t0,460
  18:   80000317                auipc   t1,0x80000
  1c:   fe830313                addi    t1,t1,-24 # 80000000 <__bss_end>
  20:   80000397                auipc   t2,0x80000
  24:   fe038393                addi    t2,t2,-32 # 80000000 <__bss_end>

00000028 <copy_data>:
  28:   00730c63                beq     t1,t2,40 <copy_done>
  2c:   0002ae03                lw      t3,0(t0)
  30:   01c32023                sw      t3,0(t1)
  34:   00428293                addi    t0,t0,4
  38:   00430313                addi    t1,t1,4
  3c:   fedff06f                j       28 <copy_data>

00000040 <copy_done>:
  40:   80000317                auipc   t1,0x80000
  44:   fc030313                addi    t1,t1,-64 # 80000000 <__bss_end>
  48:   80000397                auipc   t2,0x80000
  4c:   fb838393                addi    t2,t2,-72 # 80000000 <__bss_end>

00000050 <clear_bss>:
  50:   00730863                beq     t1,t2,60 <bss_done>
  54:   00032023                sw      zero,0(t1)
  58:   00430313                addi    t1,t1,4
  5c:   ff5ff06f                j       50 <clear_bss>

00000060 <bss_done>:
  60:   000012b7                lui     t0,0x1
  64:   80028293                addi    t0,t0,-2048 # 800 <__data_load+0x634>
  68:   3042a073                csrs    mie,t0
  6c:   00800293                li      t0,8
  70:   3002a073                csrs    mstatus,t0
  74:   130000ef                jal     1a4 <main>

00000078 <hang>:
  78:   0000006f                j       78 <hang>

0000007c <trap_handler>:
  7c:   342022f3                csrr    t0,mcause
  80:   80000337                lui     t1,0x80000
  84:   00b30313                addi    t1,t1,11 # 8000000b <__bss_end+0xb>
  88:   0a628a63                beq     t0,t1,13c <irq_handler>
  8c:   80000337                lui     t1,0x80000
  90:   00330313                addi    t1,t1,3 # 80000003 <__bss_end+0x3>
  94:   0e628863                beq     t0,t1,184 <software_handler>
  98:   80000337                lui     t1,0x80000
  9c:   00730313                addi    t1,t1,7 # 80000007 <__bss_end+0x7>
  a0:   0a628e63                beq     t0,t1,15c <timer_handler>
  a4:   00b00313                li      t1,11
  a8:   04628063                beq     t0,t1,e8 <ecall_handler>
  ac:   00300313                li      t1,3
  b0:   04628a63                beq     t0,t1,104 <ebreak_handler>
  b4:   00200313                li      t1,2
  b8:   06628463                beq     t0,t1,120 <illegal_handler>
  bc:   00100313                li      t1,1
  c0:   00628c63                beq     t0,t1,d8 <memory_error>
  c4:   00500313                li      t1,5
  c8:   00628863                beq     t0,t1,d8 <memory_error>
  cc:   00700313                li      t1,7
  d0:   00628463                beq     t0,t1,d8 <memory_error>

000000d4 <hang>:
  d4:   0000006f                j       d4 <hang>

000000d8 <memory_error>:
  d8:   04d00293                li      t0,77
  dc:   10000337                lui     t1,0x10000
  e0:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffe34>

000000e4 <memory_hang>:
  e4:   0000006f                j       e4 <memory_hang>

000000e8 <ecall_handler>:
  e8:   04500293                li      t0,69
  ec:   10000337                lui     t1,0x10000
  f0:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffe34>
  f4:   341023f3                csrr    t2,mepc
  f8:   00438393                addi    t2,t2,4
  fc:   34139073                csrw    mepc,t2
 100:   30200073                mret

00000104 <ebreak_handler>:
 104:   04b00293                li      t0,75
 108:   10000337                lui     t1,0x10000
 10c:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffe34>
 110:   341023f3                csrr    t2,mepc
 114:   00438393                addi    t2,t2,4
 118:   34139073                csrw    mepc,t2
 11c:   30200073                mret

00000120 <illegal_handler>:
 120:   04900293                li      t0,73
 124:   10000337                lui     t1,0x10000
 128:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffe34>
 12c:   341023f3                csrr    t2,mepc
 130:   00438393                addi    t2,t2,4
 134:   34139073                csrw    mepc,t2
 138:   30200073                mret

0000013c <irq_handler>:
 13c:   05100293                li      t0,81
 140:   10000337                lui     t1,0x10000
 144:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffe34>
 148:   00000293                li      t0,0
 14c:   10000337                lui     t1,0x10000
 150:   01030313                addi    t1,t1,16 # 10000010 <__data_load+0xffffe44>
 154:   00532023                sw      t0,0(t1)
 158:   30200073                mret

0000015c <timer_handler>:
 15c:   05400293                li      t0,84
 160:   10000337                lui     t1,0x10000
 164:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffe34>
 168:   c01023f3                rdtime  t2
 16c:   3e800e13                li      t3,1000
 170:   01c383b3                add     t2,t2,t3
 174:   10000337                lui     t1,0x10000
 178:   00430313                addi    t1,t1,4 # 10000004 <__data_load+0xffffe38>
 17c:   00732023                sw      t2,0(t1)
 180:   30200073                mret

00000184 <software_handler>:
 184:   05300293                li      t0,83
 188:   10000337                lui     t1,0x10000
 18c:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffe34>
 190:   00000293                li      t0,0
 194:   10000337                lui     t1,0x10000
 198:   01830313                addi    t1,t1,24 # 10000018 <__data_load+0xffffe4c>
 19c:   00532023                sw      t0,0(t1)
 1a0:   30200073                mret

000001a4 <main>:
 1a4:   04100293                li      t0,65
 1a8:   10000337                lui     t1,0x10000
 1ac:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffe34>
 1b0:   00000073                ecall
 1b4:   04200293                li      t0,66
 1b8:   00530023                sb      t0,0(t1)
 1bc:   c0001073                unimp
 1c0:   04300293                li      t0,67
 1c4:   00530023                sb      t0,0(t1)

000001c8 <loop>:
 1c8:   fddff06f                j       1a4 <main>

```

</details>

---

## Расширение M

 
Расширение **M (Integer Multiply/Divide)** добавляет к базовому набору команд (RV32I) инструкции для умножения и деления. В базовом RV32I эти операции приходится эмулировать программно, что очень медленно.

Все инструкции расширения `M` имеют тот же `R`-формат

| funct7    | rs2 (2-й регистр-источник) | rs1 (1-й регистр-источник) | funct3 (подфункция) | rd (регистр-приёмник) | opcode  |
| ---       | ---                        | ---                        | ---                 | ---                   | ---     |
| **31:25** | **24:20**                  | **19:15**                  | **14:12**           | **11:7**              | **6:0** |
| 7 бит     | 5 бит                      | 5 бит                      | 3 бита              | 5 бит                 | 7 бит   |

 

Список инструкций:


| Команда  | funct3 | funct7    | Что делает                                     | Пример              |
| -------- | ------ | --------- | ---------------------------------------------- | ------------------- |
| `mul`    | `000`  | `0000001` | Младшие 32 бита произведения: `rd = rs1 * rs2` | `mul x5, x6, x7`    |
| `mulh`   | `001`  | `0000001` | Старшие 32 бита знакового произведения         | `mulh x5, x6, x7`   |
| `mulhsu` | `010`  | `0000001` | Старшие 32 бита произведения signed × unsigned | `mulhsu x5, x6, x7` |
| `mulhu`  | `011`  | `0000001` | Старшие 32 бита беззнакового произведения      | `mulhu x5, x6, x7`  |
| `div`    | `100`  | `0000001` | Знаковое деление: `rd = rs1 / rs2`             | `div x5, x6, x7`    |
| `divu`   | `101`  | `0000001` | Беззнаковое деление                            | `divu x5, x6, x7`   |
| `rem`    | `110`  | `0000001` | Остаток от знакового деления                   | `rem x5, x6, x7`    |
| `remu`   | `111`  | `0000001` | Остаток от беззнакового деления                | `remu x5, x6, x7`   |


<br>

| Категория | Инструкции | Описание |
| :--- | :--- | :--- |
| **Умножение** | `MUL` | 32-битное умножение (младшие 32 бита результата) |
| | `MULH` | Умножение со знаком (старшие 32 бита) |
| | `MULHU` | Беззнаковое умножение (старшие 32 бита) |
| | `MULHSU` | Умножение: знаковое × беззнаковое (старшие 32 бита) |
| **Деление** | `DIV` | Деление со знаком |
| | `DIVU` | Беззнаковое деление |
| **Остаток** | `REM` | Остаток от деления со знаком |
| | `REMU` | Остаток от беззнакового деления |


<br>
<details>

<summary><b>Тест всех инструкций расширения M</b></summary>
 

Пример анализа инструкции `mul`:
```
Instr hex    : 0x026281b3
Assembly     : mul x3, x5, x6
Format       : R (M)
Instr        : 00000010011000101000000110110011
opcode       : .........................0110011 (0x33)
rd (receiver): ....................00011....... x3 ("gp")
funct3       : .................000............ (mul)
rs1 (source) : ............00101............... x5 ("t0")
rs2 (source) : .......00110.................... x6 ("t1")
funct7       : 0000001......................... (1)
```



**Файл test_m.S**
```
.global main

.section .text
main:
    # =========================================================================
    # 1. ТЕСТЫ УМНОЖЕНИЯ (MUL / MULH / MULHU / MULHSU)
    # =========================================================================

    # --- ТЕСТ MUL (Младшие 32 бита) ---
    li  t0, -1                  # 0xFFFFFFFF
    li  t1, 5
    mul x3, t0, t1              # x3 (gp) должен быть 0xFFFFFFFB

    # --- ТЕСТ MULH (Старшие 32 бита, Signed) ---
    li  t2, -1                  # 0xFFFFFFFF
    li  t3, 5
    mulh x4, t2, t3             # x4 (tp) должен быть 0xFFFFFFFF 

    # --- ТЕСТ MULHU (Старшие 32 бита, Unsigned) ---
    li  t4, -1                  # 0xFFFFFFFF
    li  t5, 5
    mulhu x17, t4, t5            # x17 должен быть 0x00000004                 

    # --- ТЕСТ MULHSU (Старшие 32 бита, Signed / Unsigned) ---
    li  s2, 0x80000000          # Отрицательное минимальное (-2147483648)
    li  s3, -1                  # Трактуется как беззнаковое 4294967295
    mulhsu x16, s2, s3          # x16 должен быть 0x80000000             


    # =========================================================================
    # 2. ТЕСТЫ ДЕЛЕНИЯ И ОСТАТКА (DIV / DIVU / REM / REMU)
    # =========================================================================

    # --- ТЕСТ DIV (Знаковое деление) ---
    # -20 / 5 = -4 (0xFFFFFFFC)
    li  t0, -20
    li  t1, 5
    div x7, t0, t1              # x7 (t2) = 0xFFFFFFFC

    # --- ТЕСТ DIVU (Беззнаковое деление) ---
    # 0xFFFFFFFE / 2 = 0x7FFFFFFF
    li  t0, -2                  # Unsigned: 4294967294
    li  t1, 2
    divu x8, t0, t1             # x8 (s0/fp) = 0x7FFFFFFF

    # --- ТЕСТ REM (Знаковый остаток) ---
    # -11 % 5 = -1 (0xFFFFFFFF)
    li  t0, -11
    li  t1, 5
    rem x9, t0, t1              # x9 (s1) = 0xFFFFFFFF

    # --- ТЕСТ REMU (Беззнаковый остаток) ---
    # 4294967295 % 10 = 5
    li  t0, -1                  # Unsigned: 4294967295
    li  t1, 10
    remu x10, t0, t1            # x10 (a0) = 0x00000005


    # =========================================================================
    # 3. ФИНАЛ
    # =========================================================================
end_loop:
    jal zero, end_loop          # Зацикливаемся, смотрим на регистры x3-x10
```





**Компиляция test_m.S**
```
riscv64-unknown-elf-gcc -march=rv32im_zicsr -mabi=ilp32 -nostdlib -T linker.ld startup.S trap.S test_m.S -o test_m.elf
```

Флаг:
* `-march=rv32im` базовые RV32I + умножение/деление
* `-march=rv32im_zicsr` базовые RV32I + умножение/деление + поддержка CSR 

**Создание чистого бинарного файла (flat binary)**
```
riscv64-unknown-elf-objcopy -O binary test_m.elf test_m.bin
```

**Создание hex-dump**  
```
(echo "v2.0 raw"; hexdump -v -e '1/4 "0x%08x\n"' test_m.bin) > test_m.hex
```

В ROM можно загрузить test_m.bin или test_m.hex.

**Дизассемблировать все секции с кодом**
```
riscv64-unknown-elf-objdump -D test_m.elf

---


000000e8 <main>:
  e8:   fff00293                li      t0,-1
  ec:   00500313                li      t1,5
  f0:   026281b3                mul     gp,t0,t1
  f4:   fff00393                li      t2,-1
  f8:   00500e13                li      t3,5
  fc:   03c39233                mulh    tp,t2,t3
 100:   fff00e93                li      t4,-1
 104:   00500f13                li      t5,5
 108:   03eeb8b3                mulhu   a7,t4,t5
 10c:   80000937                lui     s2,0x80000
 110:   fff00993                li      s3,-1
 114:   03392833                mulhsu  a6,s2,s3
 118:   fec00293                li      t0,-20
 11c:   00500313                li      t1,5
 120:   0262c3b3                div     t2,t0,t1
 124:   ffe00293                li      t0,-2
 128:   00200313                li      t1,2
 12c:   0262d433                divu    s0,t0,t1
 130:   ff500293                li      t0,-11
 134:   00500313                li      t1,5
 138:   0262e4b3                rem     s1,t0,t1
 13c:   fff00293                li      t0,-1
 140:   00a00313                li      t1,10
 144:   0262f533                remu    a0,t0,t1

00000148 <end_loop>:
 148:   0000006f                j       148 <end_loop>

```

</details>

---


## Псевдоинструкции

Псевдоинструкции не имеют собственного машинного кода, а представляют собой удобные «сокращения» для программиста, которые ассемблер автоматически разворачивает в базовые инструкции.

<details>
<summary> Основные группы псевдоинструкций RV32 </summary>

#### 1. Загрузка констант и адресов

Поскольку инструкции RISC-V имеют фиксированную длину 32 бита, невозможно загрузить 32-битное число или адрес памяти одной обычной командой. Для этого используются псевдоинструкции:

| Псевдоинструкция | Базовая инструкция (развертка) | Описание |
| --- | --- | --- |
| `li rd, immediate` | `addi rd, x0, immediate` (если < 12 бит) | Загрузка немедленного значения (Load Immediate) в регистр `rd`. Если число 32-битное, разворачивается в пару `lui` + `addi`. |
| `la rd, symbol` | **auipc+addi:**<br> `auipc rd, imm`<br> `addi rd, rd, imm`<br> **lui+addi:**<br> `lui rd, imm`<br> `addi rd, rd, imm`<br> **addi:** (if `addr <= 12 bit`)<br>`addi rd, zero, imm` | Загрузка адреса (Load Address) переменной или метки `symbol` (относительно текущего `PC`). |

 
#### 2. Перемещение данных между регистрами

В RISC-V нет отдельной команды `copy` или `move`. Вместо этого активно используется регистр `x0` (всегда равен нулю).

| Псевдоинструкция | Базовая инструкция | Описание |
| --- | --- | --- |
| `mv rd, rs` | `addi rd, rs, 0` | Копирование значения из регистра `rs` в регистр `rd`. |
 

#### 3. Арифметические и логические операции

Многие логические операции реализуются как инверсия или сравнение с нулем.

| Псевдоинструкция | Базовая инструкция | Описание |
| --- | --- | --- |
| `neg rd, rs` | `sub rd, x0, rs` | Смена знака (арифметическое отрицание): $rd = -rs$. |
| `not rd, rs` | `xori rd, rs, -1` | Побитовое отрицание (НЕ). |
| `seqz rd, rs` | `sltiu rd, rs, 1` | Установка `rd = 1`, если `rs == 0` (Set if Equal to Zero). |
| `snez rd, rs` | `sltu rd, x0, rs` | Установка `rd = 1`, если `rs != 0` (Set if Not Equal to Zero). |

 

#### 4. Условные переходы и сравнения с нулем

Базовые инструкции ветвления (`beq`, `bne`, `blt` и т.д.) всегда сравнивают два регистра. Псевдоинструкции позволяют сравнивать регистр с нулем или менять порядок операндов для удобства чтения.

| Псевдоинструкция | Базовая инструкция | Описание |
| --- | --- | --- |
| `beqz rs, offset` | `beq rs, x0, offset` | Переход, если `rs == 0`. |
| `bnez rs, offset` | `bne rs, x0, offset` | Переход, если `rs != 0`. |
| `blez rs, offset` | `bge x0, rs, offset` | Переход, если `rs <= 0`. |
| `bgez rs, offset` | `bge rs, x0, offset` | Переход, если `rs >= 0`. |
| `bltz rs, offset` | `blt rs, x0, offset` | Переход, если `rs < 0`. |
| `bgtz rs, offset` | `blt x0, rs, offset` | Переход, если `rs > 0`. |
| `bgt rs, rt, offset` | `blt rt, rs, offset` | Переход, если `rs > rt` (меняются местами). |
| `ble rs, rt, offset` | `bge rt, rs, offset` | Переход, если `rs <= rt`. |
 

#### 5. Безусловные переходы и управление подпрограммами

RISC-V использует инструкцию `jal` (Jump and Link) как для вызова функций, так и для обычных прыжков. Псевдоинструкции делают код более привычным.

| Псевдоинструкция | Базовая инструкция | Описание |
| --- | --- | --- |
| `j offset` | `jal x0, offset` | Безусловный переход по смещению (регистр возврата `x0` отбрасывает адрес). |
| `jr rs` | `jalr x0, 0(rs)` | Безусловный переход по адресу в регистре `rs`. |
| `jal offset` | `jal x1, offset` | Вызов функции (адрес возврата сохраняется в `x1` / `ra`). |
| `jalr rs` | `jalr x1, 0(rs)` | Вызов функции по адресу в регистре `rs`. |
| `ret` | `jalr x0, 0(x1)` | Возврат из функции (прыжок по адресу в `x1`/`ra`). |
| `call symbol` | `auipc x1, offset` + `jalr x1, offset(x1)` | Дальний вызов функции (в пределах $\pm2$ ГБ). |


</details>  

---

## Модули
  
<br>
<details>
<summary> <b> Модуль: Branch unit </b> </summary>
 
Интерпретирует флаги переходов B-типа
 
Для инструкций B-типа необходима поддержка операций ADD и SUB с флагами Sign, Carry, Overflow

Формирование сигнала Carry (работает с беззнаковыми числами): 
* Carry_SUB по классической реализации инвертированного заема (Inverted Borrow): `Carry_SUB = (A > B) OR (A == B)` (компаратор должен быть в режиме беззнакового сравнения (Unsigned))
* Carry_ADD для ADD просто выход переноса у блока ADD: `Carry_ADD = ADD Carry`
* Carry_Out формируется иcходя из текущей операции ALU (SUB или ADD), по умолчанию Carry=0 

Формирование сигнала Overflow. Используется исключительно при знаковых (signed) операциях (инструкции add, sub, blt, bge). Он сигнализирует о том, что результат операции вышел за пределы допустимого диапазона 32-битного знакового числа.
```
if (op == ADD or SUB)
    Overflow_ADD = ( A[31] == B[31] ) AND ( ALUResult[31] != A[31] )
    Overflow_SUB = ( A31 != B31 ) AND ( Result31 != A31 ) 
     или
    Overflow_SUB = ( A[31] != B[31] ) AND ( ALUResult[31] == B[31] )
else
    Overflow = 0
```

 
| Инструкция | funct3 | Логика           | Условие перехода                       |
| ---------- | ------ | ---              | ---                                    |
| BEQ        | `000`  | Zero flag        |`Zero == 1`                             |
| BNE        | `001`  | !Zero            |`Zero == 0`                             |
| BLT        | `100`  | signed compare   |`Sign XOR Overflow`                     |
| BGE        | `101`  | signed compare   |`NOT(Sign XOR Overflow)`                |
| BLTU       | `110`  | unsigned compare |`Carry == 0` *(Inverted Borrow)*        |
| BGEU       | `111`  | unsigned compare |`Carry == 1` *(Inverted Borrow)*        |


---

</details>


<br>
<details>
<summary> <b> Модуль: ControlUnit (Декодер команд) </b> </summary>
 
Это «дирижер» процессора. Самый важный блок.

Общие сигналы для управления регистрами и памятью

1. **`RegWrite` (Разрешение записи в регистры)**
* Зачем: Это команда для Регистрового Файла. Если на этом проводе `1`, то в конце такта процессор запишет результат вычислений в какой-то регистр.
* Пример: Для `add` или `addi` нам нужно сохранить результат, поэтому `RegWrite = 1`. Для команды перехода (`Branch`) или записи в память (`MemWrite`) ничего в регистры сохранять не нужно, поэтому там будет `0`.


2. **`ALUSrc` (Выбор входа Б для АЛУ)**
* Зачем: У АЛУ всегда два входа. Первый вход ($A$) — это всегда регистр. А вот на второй вход ($B$) можно подать либо второй регистр, либо число-константу из текста самой команды.
* Как работает: Этот провод управляет мультиплексором перед АЛУ. Если `ALUSrc = 0`, АЛУ считает два регистра (например, `add x1, x2, x3`). Если `ALUSrc = 1`, АЛУ берет регистр и константу (например, `addi x1, x2, 5`).


3. **`MemRead` (Чтение из оперативной памяти)**
* Зачем: Включает оперативную память данных (Data Memory) в режим чтения. Нужен только для одной команды — **`Load`** (загрузка из памяти в регистр). Во всех остальных командах тут жесткий `0`.


4. **`MemWrite` (Запись в оперативную память)**
* Зачем: Включает оперативную память в режим записи. Нужен только для команды **`Store`** (сохранить значение из регистра в память). Во всех остальных командах равен `0`.


5. **`MemToReg` (Что именно сохранять в регистр)**
* Зачем: Управляет мультиплексором на самом выходе процессора, который выбирает, *какие именно* данные пойдут обратно на запись в регистры.
* Как работает: Если `0` — мы записываем в регистр то, что посчитало АЛУ. Если `1` — мы записываем то, что прочитали из оперативной памяти (команда `Load`).



Сигналы управления переводами (Изменение хода программы)

6. **`Branch` (Условный переход)**
* Зачем: Загорается только тогда, когда выполняется команда ветвления (например, `beq` — перейти, если равно). Этот сигнал идет к логике управления счетчиком команд (`PC`) и говорит: «Эй, если АЛУ покажет, что числа равны, мы должны прыгнуть на другой адрес, а не идти дальше».


7. **`Jal` (Безусловный прыжок с сохранением адреса)**
* Зачем: Загорается при команде `jal` (Jump and Link). Программа должна бросить всё и прыгнуть по указанному адресу без всяких условий, но при этом запомнить в регистр адрес возврата (чтобы вернуться назад после выполнения функции).


8. **`Jalr` (Безусловный прыжок по адресу из регистра)**
* Зачем: То же самое, что и `Jal`, но адрес для прыжка берется не из текста команды, а вычисляется динамически (базовый адрес лежит в регистре).


Сигналы для специфических команд RISC-V

9. **`LUI` (Загрузка верхней части константы)**
* Зачем: Нужен для команды `lui` (Load Upper Immediate). В RISC-V нельзя одной командой записать большое 32-битное число в регистр. Команда `LUI` берет 20-битное число и заталкивает его в "верхнюю" часть регистра (с 12 по 31 биты), зануляя низ. Этот провод переключает процессор в этот особый режим.


10. **`AUIPC` (Добавить константу к текущему адресу PC)**
* Зачем: Нужен для команды `auipc`. Она берет текущий адрес, где сейчас находится процессор (`PC`), добавляет к нему большую константу и сохраняет результат в регистр. Это нужно для поиска данных в памяти относительно текущего положения программы.



Промежуточный сигнал для математики

11. **`ALUOp` (Промежуточный код операции)**
* Зачем: Это «подсказка» для Декодера АЛУ (ALU Control). `Control Unit` слишком занят общими сигналами, он не хочет вникать, нужно сейчас делать именно `add` или `sub`. Он просто говорит: «Так, сейчас у нас тип операции — чистая математика (R-тип), вот тебе код `ALUOp = 2`, дальше разбирайся сам по битам инструкции funct3 и funct7».

  `ALU` нужен четкий 4-битный код (например, `0b0010` для сложения, `0b0110` для вычитания). Но `Control Unit`, смотрит только на первых 7 бит инструкции (`opcode`) в которых нет этой информации. В инструкции есть данные про конкретную операцию сложения или вычитания но в `opcode` есть только ее обший тип `R-тип`.
  
  Поэтому ALUOp — это просто переключатель режимов для будущего декодера АЛУ. Сделаем его 2-битным (этого за глаза хватит для всех 9 команд) и зададим 4 простых режима:

  Мы соберем только единицы для Бита 0 и Бита 1 для ALUOp:
  * **`00`** — Режим для сложения (нужен для `Load`, `Store`, `Jal`, `Jalr`, `AUIPC`). АЛУ просто посчитает адрес.
  * **`01`** — Режим для вычитания (нужен для `Branch`). АЛУ вычтет числа для проверки условий.
  * **`10`** — Режим «Смотри в биты `funct3` и `funct7`» (нужен для **R-типа**).
  * **`11`** — Режим «Смотри только в биты `funct3`» (нужен для **I-типа**).

  Раз у нас по умолчанию для большинства команд (`Load`, `Store`, `Jal`, `Jalr`, `AUIPC`) режим должен быть `00`, то нам вообще не нужно трогать эти провода. 
  * Для Load и Store АЛУ должно вычислить адрес в памяти. Формула адреса в спецификации RISC-V: Базовый_Адрес + Смещение. Это обычное сложение. Режим `00` с этим справляется.
  * Для Jal процессору нужно посчитать адрес прыжка: Текущий_PC + Смещение. Снова обычное сложение. Режим `00` опять подходит.
  * Для Jalr формула адреса: Значение_из_регистра + Смещение. И снова это тупо сложение. Режим `00` опять в деле.

12. **`ImmSrc`** (Immediate Source) — это управляющий провод (обычно 2 или 3 бита), который командует ещё одним блоком процессора, который называется Extend (или Блок расширения константы). 
* В командах RISC-V константы (числа, зашитые прямо в код команды) раскиданы по 32-битной инструкции самым безумным образом ради экономии транзисторов:
  * В I-типе (например, addi) константа лежит единым куском в битах с 20 по 31.
  * В S-типе (команда sw) константа разорвана на два куска: биты 7–11 и биты 25–31.
  * В B-типе (переходы beq) биты константы вообще перемешаны в шахматном порядке для оптимизации логики переходов.

  Специальный блок расширения будет принимать всю 32-битную инструкцию, склеивать нужные биты сплиттерами и выдавать нормальное 32-битное число на вход АЛУ. Но этот блок должен знать, по какому правилу склеивать биты прямо сейчас.

  В RISC-V есть 5 разных типов констант: I, S, B, U, J.
    А наш провод ImmSrc — всего 2-битный. Двумя битами можно закодировать максимум 4 комбинации (00, 01, 10, 11). Пять типов туда физически не влезают!

  Поэтому в разных процессорах разработчики выкручиваются по-разному, стандартная классическая кодировка из книги Харрис-Харрис (Digital Design and Computer Architecture):
    * 00 — I-тип (сюда же относятся Load, Jalr, LUI и AUIPC)
    * 01 — S-тип (только Store)
    * 10 — B-тип (только Branch)
    * 11 — J-тип (только Jal)

</details>

---

## Примеры

* [Вывод строки (C)](#Вывод-строки-c)
* [Вывод строки (Rust)](#Вывод-строки-rust)
* [Framebuffer](#framebuffer)
     * [Рисование окружности алгоритмом Брезенхема](#Рисование-окружности-алгоритмом-Брезенхема)
     * [Заливка цветом без поддержки буферизированного ввода](#Заливка-цветом-без-поддержки-буферизированного-ввода)
     * [Заливка цветом с двойной буферизацией (Page Flipping)](#Заливка-цветом-с-двойной-буферизацией-page-flipping)
* [DMA](#dma)

---

 
> [The NEORV32 RISC-V Processor](https://github.com/stnolting/neorv32)
> * [Весь процессор также доступен в виде блока IP-ядер Vivado](https://stnolting.github.io/neorv32/ug/#_packaging_the_processor_as_vivado_ip_block)
> * [интеграция с Rust через фреймворк Embassy](https://crates.io/crates/embassy-neorv32)
> 
> Примеры реализации архитектуры:
> * [Single-Cycle-RISCV-Processor-using-Digital-Software](https://github.com/Anish-Rooj-cpu/Single-Cycle-RISCV-Processor-using-Digital-Software)
> * [RISC-V Project](https://github.com/RISCeirb/Risc-v-processor) ([LAST VERSION : RV32IM (WITHOUT CSR AND FENCE INSTRUCTION)](https://github.com/RISCeirb/Risc-v-processor/tree/main/RV32IM))

Доп. ресурсы:
* [ARVES, разработка процессора через VHDL](https://youtube.com/playlist?list=PLeVWfsKqC7rN0b9k1TaQej1kG4wkKPtTJ&si=ChwiTCZpolsglppi)
* [Learn RISC-V](https://github.com/riscv/learn)
* [[UNИX] (2025, весна) Архитектура и язык ассемблера RISC-V](https://www.youtube.com/playlist?list=PL6kSdcHYB3x7TqvWZDPJM_TOBbdXQkd8v)
* [[UNИX] Архитектура и язык ассемблера RISC-V (весна 2024)](https://www.youtube.com/playlist?list=PL6kSdcHYB3x5kaDr8VY9rD6gK3q-gI94I)

 
---

```
# RISC-V тулчейн

sudo apt install gcc-riscv64-unknown-elf

# После установки добавьте путь к тулчейну в ваш ~/.zshrc, если он не был добавлен автоматически:

export PATH="/usr/local/opt/riscv-gnu-toolchain/bin:$PATH"

# Проверка установки
riscv64-unknown-elf-gcc --version

```

---

### Вывод строки (C)

Так как у нас нет ОС, поэтому обычный `printf()` работать не будет (нет библиотеки), нужно сделать свой вывод символа через запись в пространство адреса MMIO (memory-mapped register), терминал вывода у нас зарегистрированн по адресу `0x10000000`
 
**Файл test_program_C.c**
```c
volatile unsigned char *uart = (unsigned char *)0x10000000;

void custom_putchar(char c)
{
    *uart = c;
}

void print_array(const char *arr, int size)
{
    for (int i = 0; i < size; i++)
    {
        custom_putchar(arr[i]);
    }
}

int main()
{
    static char message[] = "Hello C!\n";

    print_array(message, 9);

    while (1)
    {
    }

    return 0;
}
```


**Компиляция test_program_C.c**
```
riscv64-unknown-elf-gcc -march=rv32im_zicsr -mabi=ilp32 -nostdlib -T linker.ld startup.S trap.S test_program_C.c -o test_program_C.elf
```

Флаг:
* `-march=rv32im` базовые RV32I + умножение/деление
* `-march=rv32im_zicsr` базовые RV32I + умножение/деление + поддержка CSR 

**Создание чистого бинарного файла (flat binary)**
```
riscv64-unknown-elf-objcopy -O binary test_program_C.elf test_program_C.bin
```

**Создание hex-dump**  
```
(echo "v2.0 raw"; hexdump -v -e '1/4 "0x%08x\n"' test_program_C.bin) > test_program_C.hex
```

В ROM можно загрузить test_program_C.bin или test_program_C.hex 


**Размеры секций .text, .data, .bss и общий размер**
```
size test_program_C.elf
---
   text    data     bss     dec     hex filename
    612      16       0     628     274 test_program_C.elf
```

В секции .data содержится 16 байт: 

```
H   1
e   1
l   1
l   1
o   1
' ' 1
C   1
!   1
\n  1
\0  1
----------
10 байт
```

И линкер выравнивает секцию .data добавляя +6 байт заполнителя (padding)

В секции .text 612 байт это 153 инструкции (всех вместе и startup и main)


```
riscv64-unknown-elf-readelf -x .data test_program_C.elf
---

Hex dump of section '.data':
  0x80000000 48656c6c 6f204321 0a000000 00000010 Hello C!........
```

**Заголовки секций**

```
riscv64-unknown-elf-objdump -h test_program_C.elf
---

test_program_C.elf:     file format elf32-littleriscv

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .text         00000264  00000000  00000000  00001000  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  1 .data         00000010  80000000  00000264  00002000  2**2
                  CONTENTS, ALLOC, LOAD, DATA
  2 .bss          00000000  80000010  00000274  00002010  2**0
                  ALLOC
  3 .riscv.attributes 00000037  00000000  00000000  00002010  2**0
                  CONTENTS, READONLY
  4 .comment      00000022  00000000  00000000  00002047  2**0
                  CONTENTS, READONLY
```

LMA (Load Memory Address) это адрес, где эти данные лежат в образе прошивки. Для секции `.data LMA = 00000264` именно отсюда код startup скопирует `static char message[] = "Hello C!\n";` из ROM[0x264].

VMA (Virtual Memory Address) это адрес, по которому программа думает, что данные находятся во время выполнения. Для секции `.data VMA = 0x80000000` это значит переменная counter имеет адрес `0x80000000` и инструкция чтения обратится к `RAM[0x80000000]`.
 

 
<details>
<summary><b>Дизассемблировать все секции с кодом</b></summary>

```
riscv64-unknown-elf-objdump -D test_program_C.elf

---

test_program_C.elf:     file format elf32-littleriscv


Disassembly of section .text:

00000000 <_start>:
   0:   80800117                auipc   sp,0x80800
   4:   00010113                mv      sp,sp
   8:   00000297                auipc   t0,0x0
   c:   07428293                addi    t0,t0,116 # 7c <trap_handler>
  10:   30529073                csrw    mtvec,t0
  14:   26400293                li      t0,612
  18:   80000317                auipc   t1,0x80000
  1c:   fe830313                addi    t1,t1,-24 # 80000000 <message.0>
  20:   80000397                auipc   t2,0x80000
  24:   ff038393                addi    t2,t2,-16 # 80000010 <__bss_end>

00000028 <copy_data>:
  28:   00730c63                beq     t1,t2,40 <copy_done>
  2c:   0002ae03                lw      t3,0(t0)
  30:   01c32023                sw      t3,0(t1)
  34:   00428293                addi    t0,t0,4
  38:   00430313                addi    t1,t1,4
  3c:   fedff06f                j       28 <copy_data>

00000040 <copy_done>:
  40:   80000317                auipc   t1,0x80000
  44:   fd030313                addi    t1,t1,-48 # 80000010 <__bss_end>
  48:   80000397                auipc   t2,0x80000
  4c:   fc838393                addi    t2,t2,-56 # 80000010 <__bss_end>

00000050 <clear_bss>:
  50:   00730863                beq     t1,t2,60 <bss_done>
  54:   00032023                sw      zero,0(t1)
  58:   00430313                addi    t1,t1,4
  5c:   ff5ff06f                j       50 <clear_bss>

00000060 <bss_done>:
  60:   000012b7                lui     t0,0x1
  64:   80028293                addi    t0,t0,-2048 # 800 <__data_load+0x59c>
  68:   3042a073                csrs    mie,t0
  6c:   00800293                li      t0,8
  70:   3002a073                csrs    mstatus,t0
  74:   1cc000ef                jal     240 <main>

00000078 <hang>:
  78:   0000006f                j       78 <hang>

0000007c <trap_handler>:
  7c:   342022f3                csrr    t0,mcause
  80:   80000337                lui     t1,0x80000
  84:   00b30313                addi    t1,t1,11 # 8000000b <message.0+0xb>
  88:   0a628a63                beq     t0,t1,13c <irq_handler>
  8c:   80000337                lui     t1,0x80000
  90:   00330313                addi    t1,t1,3 # 80000003 <message.0+0x3>
  94:   0e628863                beq     t0,t1,184 <software_handler>
  98:   80000337                lui     t1,0x80000
  9c:   00730313                addi    t1,t1,7 # 80000007 <message.0+0x7>
  a0:   0a628e63                beq     t0,t1,15c <timer_handler>
  a4:   00b00313                li      t1,11
  a8:   04628063                beq     t0,t1,e8 <ecall_handler>
  ac:   00300313                li      t1,3
  b0:   04628a63                beq     t0,t1,104 <ebreak_handler>
  b4:   00200313                li      t1,2
  b8:   06628463                beq     t0,t1,120 <illegal_handler>
  bc:   00100313                li      t1,1
  c0:   00628c63                beq     t0,t1,d8 <memory_error>
  c4:   00500313                li      t1,5
  c8:   00628863                beq     t0,t1,d8 <memory_error>
  cc:   00700313                li      t1,7
  d0:   00628463                beq     t0,t1,d8 <memory_error>

000000d4 <hang>:
  d4:   0000006f                j       d4 <hang>

000000d8 <memory_error>:
  d8:   04d00293                li      t0,77
  dc:   10000337                lui     t1,0x10000
  e0:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffd9c>

000000e4 <memory_hang>:
  e4:   0000006f                j       e4 <memory_hang>

000000e8 <ecall_handler>:
  e8:   04500293                li      t0,69
  ec:   10000337                lui     t1,0x10000
  f0:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffd9c>
  f4:   341023f3                csrr    t2,mepc
  f8:   00438393                addi    t2,t2,4
  fc:   34139073                csrw    mepc,t2
 100:   30200073                mret

00000104 <ebreak_handler>:
 104:   04b00293                li      t0,75
 108:   10000337                lui     t1,0x10000
 10c:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffd9c>
 110:   341023f3                csrr    t2,mepc
 114:   00438393                addi    t2,t2,4
 118:   34139073                csrw    mepc,t2
 11c:   30200073                mret

00000120 <illegal_handler>:
 120:   04900293                li      t0,73
 124:   10000337                lui     t1,0x10000
 128:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffd9c>
 12c:   341023f3                csrr    t2,mepc
 130:   00438393                addi    t2,t2,4
 134:   34139073                csrw    mepc,t2
 138:   30200073                mret

0000013c <irq_handler>:
 13c:   05100293                li      t0,81
 140:   10000337                lui     t1,0x10000
 144:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffd9c>
 148:   00000293                li      t0,0
 14c:   10000337                lui     t1,0x10000
 150:   01030313                addi    t1,t1,16 # 10000010 <__data_load+0xffffdac>
 154:   00532023                sw      t0,0(t1)
 158:   30200073                mret

0000015c <timer_handler>:
 15c:   05400293                li      t0,84
 160:   10000337                lui     t1,0x10000
 164:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffd9c>
 168:   c01023f3                rdtime  t2
 16c:   3e800e13                li      t3,1000
 170:   01c383b3                add     t2,t2,t3
 174:   10000337                lui     t1,0x10000
 178:   00430313                addi    t1,t1,4 # 10000004 <__data_load+0xffffda0>
 17c:   00732023                sw      t2,0(t1)
 180:   30200073                mret

00000184 <software_handler>:
 184:   05300293                li      t0,83
 188:   10000337                lui     t1,0x10000
 18c:   00530023                sb      t0,0(t1) # 10000000 <__data_load+0xffffd9c>
 190:   00000293                li      t0,0
 194:   10000337                lui     t1,0x10000
 198:   01830313                addi    t1,t1,24 # 10000018 <__data_load+0xffffdb4>
 19c:   00532023                sw      t0,0(t1)
 1a0:   30200073                mret

000001a4 <custom_putchar>:
 1a4:   fe010113                addi    sp,sp,-32 # 807fffe0 <__bss_end+0x7fffd0>
 1a8:   00812e23                sw      s0,28(sp)
 1ac:   02010413                addi    s0,sp,32
 1b0:   00050793                mv      a5,a0
 1b4:   fef407a3                sb      a5,-17(s0)
 1b8:   800007b7                lui     a5,0x80000
 1bc:   00c7a783                lw      a5,12(a5) # 8000000c <uart>
 1c0:   fef44703                lbu     a4,-17(s0)
 1c4:   00e78023                sb      a4,0(a5)
 1c8:   00000013                nop
 1cc:   01c12403                lw      s0,28(sp)
 1d0:   02010113                addi    sp,sp,32
 1d4:   00008067                ret

000001d8 <print_array>:
 1d8:   fd010113                addi    sp,sp,-48
 1dc:   02112623                sw      ra,44(sp)
 1e0:   02812423                sw      s0,40(sp)
 1e4:   03010413                addi    s0,sp,48
 1e8:   fca42e23                sw      a0,-36(s0)
 1ec:   fcb42c23                sw      a1,-40(s0)
 1f0:   fe042623                sw      zero,-20(s0)
 1f4:   0280006f                j       21c <print_array+0x44>
 1f8:   fec42783                lw      a5,-20(s0)
 1fc:   fdc42703                lw      a4,-36(s0)
 200:   00f707b3                add     a5,a4,a5
 204:   0007c783                lbu     a5,0(a5)
 208:   00078513                mv      a0,a5
 20c:   f99ff0ef                jal     1a4 <custom_putchar>
 210:   fec42783                lw      a5,-20(s0)
 214:   00178793                addi    a5,a5,1
 218:   fef42623                sw      a5,-20(s0)
 21c:   fec42703                lw      a4,-20(s0)
 220:   fd842783                lw      a5,-40(s0)
 224:   fcf74ae3                blt     a4,a5,1f8 <print_array+0x20>
 228:   00000013                nop
 22c:   00000013                nop
 230:   02c12083                lw      ra,44(sp)
 234:   02812403                lw      s0,40(sp)
 238:   03010113                addi    sp,sp,48
 23c:   00008067                ret

00000240 <main>:
 240:   ff010113                addi    sp,sp,-16
 244:   00112623                sw      ra,12(sp)
 248:   00812423                sw      s0,8(sp)
 24c:   01010413                addi    s0,sp,16
 250:   00900593                li      a1,9
 254:   800007b7                lui     a5,0x80000
 258:   00078513                mv      a0,a5
 25c:   f7dff0ef                jal     1d8 <print_array>
 260:   0000006f                j       260 <main+0x20>

Disassembly of section .data:

80000000 <message.0>:
80000000:       6548                    .insn   2, 0x6548
80000002:       6c6c                    .insn   2, 0x6c6c
80000004:       2143206f                j       80032218 <__data_end+0x32208>
80000008:       000a                    .insn   2, 0x000a
        ...

8000000c <uart>:
8000000c:       0000                    .insn   2, 0x
8000000e:       1000                    .insn   2, 0x1000

```

---

</details>

<br>
<video controls width="100%" muted playsinline preload="metadata">
    <source src="/Computer-Science-Bookshelf/img/riscv/test_program_C.mp4" type="video/mp4">
    Ваш браузер не поддерживает видео.
</video>

---

### Вывод строки (Rust)

**Файл test_program_Rust.rs**

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

static MESSAGE: &[u8] = b"Hello Rust!\n";

fn putchar(c: u8) {
    unsafe {
        *(0x10000000 as *mut u8) = c;
    }
}

fn print_array(data: &[u8]) {
    for &b in data {
        putchar(b);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    print_array(MESSAGE);

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
```

`MESSAGE` попадёт не в `.data`, а в `.rodata`, потому что она неизменяемая. 

> Но если использовать `static mut MESSAGE: [u8; 9] = *b"Hello Rust!\n";` то массив станет изменяемым и попадёт в `.data`


**Компиляция test_program_Rust.rs**
```
rustc --target=riscv32i-unknown-none-elf -C target-feature=+m,+zicsr -C opt-level=s -C panic=abort --emit=obj test_program_Rust.rs
```    


**Линковка**
```
riscv64-unknown-elf-as -march=rv32im_zicsr -mabi=ilp32 startup.S -o startup.o
riscv64-unknown-elf-as -march=rv32im_zicsr -mabi=ilp32 trap.S -o trap.o

riscv64-unknown-elf-ld -m elf32lriscv -T linker.ld startup.o trap.o test_program_Rust.o -o test_program_Rust.elf
```    

**Создание чистого бинарного файла (flat binary)**
```
riscv64-unknown-elf-objcopy -O binary test_program_Rust.elf test_program_Rust.bin
```    

**Размеры секций .text, .data, .bss и общий размер**
```
riscv64-unknown-elf-size test_program_Rust.elf
---
   text    data     bss     dec     hex filename
    472       0       0     472     1d8 test_program_Rust.elf
```



**Заголовки секций**

```
riscv64-unknown-elf-objdump -h test_program_Rust.elf
---

test_program_Rust.elf:     file format elf32-littleriscv

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .text         000001cc  00000000  00000000  00001000  2**2
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  1 .rodata       0000000c  000001cc  000001cc  000011cc  2**0
                  CONTENTS, ALLOC, LOAD, READONLY, DATA
  2 .data         00000000  80000000  000001d8  00002000  2**0
                  CONTENTS, ALLOC, LOAD, DATA
  3 .bss          00000000  80000000  000001d8  00002000  2**0
                  ALLOC
  4 .riscv.attributes 00000037  00000000  00000000  00002000  2**0
                  CONTENTS, READONLY
  5 .comment      0000002c  00000000  00000000  00002037  2**0
                  CONTENTS, READONLY
```

Строка находится в `.rodata` `VMA` и `LMA` адрес `0x000001cc` и используются для чтения прямо из `ROM[0x000001cc]` (метаданные с размером и данными, так как мы используем ссылку `&[u8]`) без необходимости копировать их в RAM.

```
ROM

0x00000000
+----------------+
| .text          |
| 0x1cc байт     |
+----------------+
| .rodata        |
| 9 байт         |
| "Hello C!\n"   |
+----------------+
```

**Проверить, что есть символы main:**
```
riscv64-unknown-elf-nm test_program_Rust.o | grep main
---
00000000 T main
```


<br>
<video controls width="100%" muted playsinline preload="metadata">
    <source src="/Computer-Science-Bookshelf/img/riscv/test_program_Rust.mp4" type="video/mp4">
    Ваш браузер не поддерживает видео.
</video>
  
---

### Framebuffer

Встроенный компонент `Graphic RAM` (Видеопамять) сочетает в себе сразу и массив RAM, и внутренний графический контроллер, и сам экран.

Компонент `Graphic RAM` в симуляторе Digital, имеет палитру цвета:
* 0–9 — основные цвета
* 10–31 — свободно (можно переопределить в палитре)
* 32–63 — градации серого (32 - почти черный, 63 - почти белый)
* 64–127 — RGB222 (по 2 бита на канал т.е. `RR*GG*BB=4*4*4=64` цвета), цвет задается прямо индексом, без палитры.

Цвет занимает 8 бит на пиксель.

| Индекс | Цвет                                                          |
| -----: | ------------------------------------------------------------- |
|      0 | Белый                                                         |
|      1 | Черный                                                        |
|      2 | Красный                                                       |
|      3 | Зеленый                                                       |
|      4 | Синий                                                         |
|      5 | Желтый                                                        |
|      6 | Циан (Морской волны)                                          |
|      7 | Маджента (Фуксия, Пурпурный) RGB (255,0,255), HEX: #ff00ff  |
|      8 | Цвет Яндекса RGB (255,200,0), HEX: #ffc800                  |
|      9 | Светлый пурпурно-розовый 	RGB (255,175,175), HEX: #ffafaf  |
 

Без палитры используется значения 64–127 
```
Формат RGB222:
Индекс = 0b01RRGGBB (старшие 2 бита всегда 01 для режима RGB222)

```
 
---

### Рисование окружности алгоритмом Брезенхема

<br>
<details>
<summary> <b> Файл test_сircle.rs </b> </summary>
  
```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

const FRAMEBUFFER: *mut u8 = 0x9000_0000 as *mut u8;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[inline(always)]
fn put_pixel(x: i32, y: i32, color: u8) {
    if x < 0 || y < 0 {
        return;
    }
    if x >= WIDTH as i32 || y >= HEIGHT as i32 {
        return;
    }
    unsafe {
        FRAMEBUFFER
            .add(y as usize * WIDTH + x as usize)
            .write_volatile(color);
    }
}

// Рисуем 8 симметричных точек окружности
#[inline(always)]
fn circle_points(
    xc: i32,
    yc: i32,
    x: i32,
    y: i32,
    color: u8
) {
    put_pixel(xc + x, yc + y, color);
    put_pixel(xc - x, yc + y, color);

    put_pixel(xc + x, yc - y, color);
    put_pixel(xc - x, yc - y, color);

    put_pixel(xc + y, yc + x, color);
    put_pixel(xc - y, yc + x, color);

    put_pixel(xc + y, yc - x, color);
    put_pixel(xc - y, yc - x, color);
}

fn draw_circle(
    xc: i32,
    yc: i32,
    radius: i32,
    color: u8
) {

    let mut x = 0;
    let mut y = radius;

    // Начальная ошибка
    let mut d = 3 - 2 * radius;
    while x <= y {
        circle_points(
            xc,
            yc,
            x,
            y,
            color
        );

        if d < 0 {
            d += 4 * x + 6;
        } else {
            d += 4 * (x - y) + 10;
            y -= 1;
        }
        x += 1;
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Большая окружность
    draw_circle(
        160,
        100,
        70,
        10
    );
    // маленькие окружности для теста
    draw_circle(
        80,
        50,
        30,
        20
    );
    draw_circle(
        240,
        150,
        20,
        30
    );
    loop {}
}
```

</details>


**Компиляция test_сircle.rs**
```
rustc --target=riscv32i-unknown-none-elf -C target-feature=+m,+zicsr -C opt-level=s -C panic=abort --emit=obj test_сircle.rs
```
 
**Линковка**
```
riscv64-unknown-elf-as -march=rv32im_zicsr -mabi=ilp32 startup.S -o startup.o
riscv64-unknown-elf-as -march=rv32im_zicsr -mabi=ilp32 trap.S -o trap.o

riscv64-unknown-elf-ld -m elf32lriscv -T linker.ld startup.o trap.o test_сircle.o -o test_сircle.elf
```    


**Создание чистого бинарного файла (flat binary)**
```
riscv64-unknown-elf-objcopy -O binary test_сircle.elf test_сircle.bin
```    

<br>
<video controls width="100%" muted playsinline preload="metadata">
    <source src="/Computer-Science-Bookshelf/img/riscv/test_сircle.mp4" type="video/mp4">
    Ваш браузер не поддерживает видео.
</video>
  

---

### Заливка цветом без поддержки буферизированного ввода


<br> 
<details>
<summary> Заливка цветом (asm код)</summary>

**Файл test_framebuffer.S**

```asm
.section .text
.global main

##################################################
# main
# Fill 320x200 framebuffer with green
##################################################

main:

    # t0 = 0x90000000
    lui  t0, 0x90000

    # t1 = 64000 pixels (0xFA00)
    lui  t1, 0x10
    addi t1, t1, -1536

    # t2 = green color index 0x02
    addi t2, zero, 2

fill_loop:

    sb   t2, 0(t0)

    addi t0, t0, 1
    addi t1, t1, -1

    bne  t1, zero, fill_loop

hang:

    jal  zero, hang
```
 
**Компиляция test_framebuffer.S**
```
riscv64-unknown-elf-gcc -march=rv32im_zicsr -mabi=ilp32 -nostdlib -T linker.ld startup.S trap.S test_framebuffer.S -o test_framebuffer.elf
```
 
**Создание чистого бинарного файла (flat binary)**
```
riscv64-unknown-elf-objcopy -O binary test_framebuffer.elf test_framebuffer.bin
```

---

</details>


Ассемблерный код показывает свою неинформативность в высокоуровневых задачах, поэтому воспользуемся языком Rust/С.

**Файл test_framebuffer.rs**

```rust,no_run
#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

const FRAMEBUFFER: *mut u8 = 0x9000_0000 as *mut u8;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
fn put_pixel(x: usize, y: usize, color: u8) {
    if x >= WIDTH || y >= HEIGHT {
        return;
    }

    unsafe {
        FRAMEBUFFER
            .add(y * WIDTH + x)
            .write_volatile(color);
    }
}
fn fill(color: u8) {
    unsafe {
        for i in 0..(WIDTH * HEIGHT) {
            FRAMEBUFFER.add(i).write_volatile(color);
        }
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    fill(0);
    fill(1);
    fill(2);
    fill(3);
    fill(4);
    fill(5);
    fill(6);
    fill(7);
    fill(8);
    fill(9);

    loop {}
}
```


**Компиляция test_framebuffer.rs**
```
rustc --target=riscv32i-unknown-none-elf -C target-feature=+m,+zicsr -C opt-level=s -C panic=abort --emit=obj test_framebuffer.rs
```    


**Линковка**
```
riscv64-unknown-elf-as -march=rv32im_zicsr -mabi=ilp32 startup.S -o startup.o
riscv64-unknown-elf-as -march=rv32im_zicsr -mabi=ilp32 trap.S -o trap.o

riscv64-unknown-elf-ld -m elf32lriscv -T linker.ld startup.o trap.o test_framebuffer.o -o test_framebuffer.elf
```    

**Создание чистого бинарного файла (flat binary)**
```
riscv64-unknown-elf-objcopy -O binary test_framebuffer.elf test_framebuffer.bin
```    


Так происходит заливка цветов кадра 320x200 пикселей, без поддержки буферизированного ввода. 
Т.е. работает один буфер `B=0` и мы наблюдаем его наполнение - эффект «разрыва кадра» (tearing) и неприятное мерцание при более частой смене кадра.
<br>
<video controls width="100%" muted playsinline preload="metadata">
    <source src="/Computer-Science-Bookshelf/img/riscv/Framebuffer_fill_B0.mp4" type="video/mp4">
    Ваш браузер не поддерживает видео.
</video>

---

### Заливка цветом с двойной буферизацией (Page Flipping)

Внутри компонента Graphic RAM физически есть два одинаковых массива памяти (Буфер 0 и Буфер 1). У них один общий вход записи (D) и одна общая шина адреса (A).

Пока идет запись в один из буферов:
* Для записи в Буфер 0 необходимо писать по адресам от 0x9000_0000 до 0x9000_F9FF
* Для записи в Буфер 1 необходимо писать по адресам от 0x9000_FA00 до 0x9001_F3FF

Мы можем показывать противоположный уже заполненный буфер, через переключение входа `B` (Buffer Select)

Аппаратная доработка модуля `VideoRAM`: чтобы управляющая программа могла переключать буферы, вход `B` (Buffer Select) нужно вывести на регистр управления (MMIO) например на адрес `0x1000000C`, запись по нему будет переключать вход `B` 

<br>
<details>
<summary> <b> Файл test_framebuffer_page_flipping.rs </b> </summary>

```rust
#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;
const FRAME_SIZE: usize = WIDTH * HEIGHT; // 64 000 байт (0xFA00)

// Базовый адрес видеопамяти
const BASE_FRAMEBUFFER: *mut u8 = 0x9000_0000 as *mut u8;

// Регистр управления входом B (Display Control)
const DISP_CTRL: *mut u32 = 0x1000_000C as *mut u32;

// Глобальная переменная для отслеживания текущего ПОКАЗЫВАЕМОГО буфера (0 или 1)
static mut ACTIVE_BUFFER: u8 = 0;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Возвращает указатель на СКУКРЫТЫЙ (фоновый) буфер, в который сейчас можно рисовать
fn get_back_buffer() -> *mut u8 {
    unsafe {
        if ACTIVE_BUFFER == 0 {
            // Если экран показывает Буфер 0, рисуем в Буфер 1
            BASE_FRAMEBUFFER.add(FRAME_SIZE)
        } else {
            // Если экран показывает Буфер 1, рисуем в Буфер 0
            BASE_FRAMEBUFFER
        }
    }
}

/// Переключает кадры: обновляет регистр MMIO и меняет активный буфер
fn swap_buffers() {
    unsafe {
        // Инвертируем активный буфер (0 -> 1 или 1 -> 0)
        ACTIVE_BUFFER ^= 1;

        // Записываем новое значение в регистр 0x1000_000C, переключая вход B
        DISP_CTRL.write_volatile(ACTIVE_BUFFER as u32);
    }
}

fn put_pixel(x: usize, y: usize, color: u8) {
    if x >= WIDTH || y >= HEIGHT {
        return;
    }

    unsafe {
        get_back_buffer()
            .add(y * WIDTH + x)
            .write_volatile(color);
    }
}

fn fill(color: u8) {
    let back_buffer = get_back_buffer();
    unsafe {
        for i in 0..FRAME_SIZE {
            back_buffer.add(i).write_volatile(color);
        }
    }
    // Переключаем экран ТОЛЬКО ПОСЛЕ того, как кадр полностью закрашен
    swap_buffers();
}

fn fill_fast(color: u8) {
   let mut ptr = get_back_buffer(); // *mut u8
    
    // 64 000 / 16 = 4 000 итераций вместо 64 000
    let total_chunks = (WIDTH * HEIGHT) / 16; 

    unsafe {
        for _ in 0..total_chunks {
            // Пишем 16 байт подряд за одну итерацию
            ptr.offset(0).write_volatile(color);
            ptr.offset(1).write_volatile(color);
            ptr.offset(2).write_volatile(color);
            ptr.offset(3).write_volatile(color);
            ptr.offset(4).write_volatile(color);
            ptr.offset(5).write_volatile(color);
            ptr.offset(6).write_volatile(color);
            ptr.offset(7).write_volatile(color);
            ptr.offset(8).write_volatile(color);
            ptr.offset(9).write_volatile(color);
            ptr.offset(10).write_volatile(color);
            ptr.offset(11).write_volatile(color);
            ptr.offset(12).write_volatile(color);
            ptr.offset(13).write_volatile(color);
            ptr.offset(14).write_volatile(color);
            ptr.offset(15).write_volatile(color);

            ptr = ptr.add(16);
        }
    }
    swap_buffers();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Инициализация: сбрасываем регистр управления в 0
    unsafe { DISP_CTRL.write_volatile(0);}

    // Теперь каждый вызов fill() незаметно заполняет скрытый буфер 
    // и мгновенно выводит его на экран 
    fill_fast(1);
    fill_fast(2);
    fill_fast(3);
    fill_fast(4);
    fill_fast(5);
    fill_fast(6);
    fill_fast(7);
    fill_fast(8);
    fill_fast(9);
   
    loop {}
}
```

</details>

**Компиляция test_framebuffer_page_flipping.rs**
```
rustc --target=riscv32i-unknown-none-elf -C target-feature=+m,+zicsr -C opt-level=s -C panic=abort --emit=obj test_framebuffer_page_flipping.rs
```    


**Линковка**
```
riscv64-unknown-elf-as -march=rv32im_zicsr -mabi=ilp32 startup.S -o startup.o
riscv64-unknown-elf-as -march=rv32im_zicsr -mabi=ilp32 trap.S -o trap.o

riscv64-unknown-elf-ld -m elf32lriscv -T linker.ld startup.o trap.o test_framebuffer_page_flipping.o -o test_framebuffer_page_flipping.elf
```    

**Создание чистого бинарного файла (flat binary)**
```
riscv64-unknown-elf-objcopy -O binary test_framebuffer_page_flipping.elf test_framebuffer_page_flipping.bin
```    

Так происходит заливка цвета кадра 320x200 пикселей, с поддержкой двойной буферизации (page flipping). 
Т.е. работают два буфера `B=0` и `B=1` и мы не видим наполнение кадра, но количество инструкций для отрисовки кадра не позволяет иметь плавную смену кадра.
<br>
<video controls width="100%" muted playsinline preload="metadata">
    <source src="/Computer-Science-Bookshelf/img/riscv/Framebuffer_fill_page_flipping.mp4" type="video/mp4">
    Ваш браузер не поддерживает видео.
</video>
  

Листинг дизассемблированного участка кода цикла ф-ции `fill`:
```
riscv64-unknown-elf-objdump -D test_framebuffer_page_flipping.elf
---

1dc: 00d58023   sb   a3, 0(a1)    # 1. Полезная работа: пишем 1 байт цвета по адресу из a1
1e0: 00158593   addi a1, a1, 1    # 2. Инкремент адреса: a1 = a1 + 1 (переход к след. пикселю)
1e4: fec59ce3   bne  a1, a2, 1dc  # 3. Проверка: если a1 != a2, прыгаем назад на адрес 0x1dc
```

На 1 (33%) полезную инструкцию записи (sb) приходится 2 служебные инструкции (`addi` и `bne`) для обслуживания цикла, и так *64000* раз подряд!.
Процессор выполняет $64\,000 \times 3 = \mathbf{192\,000}$ инструкций/тактов только ради закраски одного кадра, из которых $128\,000$ инструкций уходило на продвижение указателя и проверку условия.

Для ускорения частоты смены кадров, нам нужно сократить количество инструкций для обслуживания цикла. Применим оптимизацию **развертка цикла (Loop Unrolling)**, развернув цикл на 16 операций записи *подряд*. Поскольку инструкция `sb` умеет прибавлять постоянное смещение к базовому регистру прямо во время выполнения, компилятору не нужно вычислять промежуточные адреса без необходимости выполнять инкремент адреса на каждом шаге. Число 16 выбрано как оптимальный баланс между эффективностью цикла и финальным размером прошивки.

Функция с разверткой цикла:

```rust,editable,no_run
fn fill_fast(color: u8) {
   let mut ptr = get_back_buffer(); // *mut u8
    
    // 64 000 / 16 = 4 000 итераций вместо 64 000
    let total_chunks = (WIDTH * HEIGHT) / 16; 

    unsafe {
        for _ in 0..total_chunks {
            // Пишем 16 байт подряд за одну итерацию
            ptr.offset(0).write_volatile(color);
            ptr.offset(1).write_volatile(color);
            ptr.offset(2).write_volatile(color);
            ptr.offset(3).write_volatile(color);
            ptr.offset(4).write_volatile(color);
            ptr.offset(5).write_volatile(color);
            ptr.offset(6).write_volatile(color);
            ptr.offset(7).write_volatile(color);
            ptr.offset(8).write_volatile(color);
            ptr.offset(9).write_volatile(color);
            ptr.offset(10).write_volatile(color);
            ptr.offset(11).write_volatile(color);
            ptr.offset(12).write_volatile(color);
            ptr.offset(13).write_volatile(color);
            ptr.offset(14).write_volatile(color);
            ptr.offset(15).write_volatile(color);

            ptr = ptr.add(16);
        }
    }
    swap_buffers();
}
```

Листинг дизассемблированного участка кода цикла ф-ции `fill_fast`:
```
 1d0:   00a58023                sb      a0,0(a1) # Базовый адрес + 0
 1d4:   00a580a3                sb      a0,1(a1) # Базовый адрес + 1 байт
 1d8:   00a58123                sb      a0,2(a1) # Базовый адрес + 2 байта
 1dc:   00a581a3                sb      a0,3(a1) # ...
 1e0:   00a58223                sb      a0,4(a1)
 1e4:   00a582a3                sb      a0,5(a1)
 1e8:   00a58323                sb      a0,6(a1)
 1ec:   00a583a3                sb      a0,7(a1)
 1f0:   00a58423                sb      a0,8(a1)
 1f4:   00a584a3                sb      a0,9(a1)
 1f8:   00a58523                sb      a0,10(a1)
 1fc:   00a585a3                sb      a0,11(a1)
 200:   00a58623                sb      a0,12(a1)
 204:   00a586a3                sb      a0,13(a1)
 208:   00a58723                sb      a0,14(a1) # Базовый адрес + 14 байт
 20c:   00a587a3                sb      a0,15(a1) # Базовый адрес + 15 байт
 210:   01058593                addi    a1,a1,16  # Увеличиваем базовый адрес a1 сразу на 16!
 214:   fac59ee3                bne     a1,a2,1d0 # Если не дошли до конца (a2), возвращаемся на 1d0 
``` 

Из 18 инструкций внутри тела цикла, 16 инструкций (89%) занимаются исключительно записью пикселей в VRAM.

Процессор выполняет $64\,000 / 16 = 4\,000$ итераций по 18 инструкций в каждой, итого $4\,000 \times 18 = \mathbf{72\,000 \text{ тактов}}$ на кадр, что делает закраску почти в 2.7 раза быстрее. Но по факту, до синего цвета оптимизированный подход дошел за 5 секунд, а не оптимизированный за 25 секунд, т.е. эффективность в 5 раз!

<br>
<video controls width="100%" muted playsinline preload="metadata">
    <source src="/Computer-Science-Bookshelf/img/riscv/Framebuffer_fill_page_flipping_LoopUnrolling.mp4" type="video/mp4">
    Ваш браузер не поддерживает видео.
</video>

---

### DMA

DMA (Direct Memory Access/Прямой доступ к памяти) *"Memory-to-Memory"* - инструмент копирования больших кусков памяти.
   
Процессор не оптимизирован для простого копирования больших блоков памяти. Он делает это с накладными расходами на выборку и декодирование каждой инструкции. Перекладывание работы на DMA: вы настраиваете DMA программно, указывая адрес источника (кадр в памяти), адрес назначения (framebuffer) и размер данных, после чего DMA берет работу на себя аппаратно копируя данные из RAM в VRAM, в это время процессор полностью свободен для выполнение логики программы.

Сперва нам необходимо загрузить источник данных т.е. RAM (так как мы не реализовывали V-расширение векторных инструкций) используя *три* иснтрукции `lui+addi+sw` для готового 32 битного числа, что покроет 4 пикселя 8-ми битного цвета. Это была шутка 🤪, если мы так сделаем, то процессор будет полностью занят программной генерацией данных (пусть даже и копированием готового видео) в RAM, что бы потом DMA ее снова копировало и занимало шину. Нам же, нужно напрямую обращаться к памяти. Для этого компьютер использует периферию, например HDD аналог которого в Digital это просто ROM с готовыми данными, которые DMA напрямую скопирует от и до, в видеопамять. 

**Тестовый модуль VideoDMA**

**Вариант ч/б** для палитры компонента `Graphic RAM` 32–63 — градации серого (32 - почти черный, 63 - почти белый)

Подготовка видеоряда к разрешению 320x200 с форматом gray, длительностью 15 секунд.
```
ffmpeg -ss 00:00:00 -i TomAndJerry.mp4 -t 15 -vf "scale=320:200,fps=15" -pix_fmt gray -f rawvideo video_320x200_gray.raw
```

Файл img/riscv/video/convert_video_gray.py для конвертации данных  
```py
import math

def convert_video_gray_raw_to_bin(raw_filename, bin_filename):
    # Кадр raw gray: 320 x 200 пикселей * 1 байт = 64 000 байт
    FRAME_SIZE = 320 * 200

    with open(raw_filename, "rb") as f_in, open(bin_filename, "wb") as f_out:
        total_pixels = 0
        
        while chunk := f_in.read(FRAME_SIZE):
            # Пропускаем неполный кадр, если файл закончился раньше времени
            if len(chunk) < FRAME_SIZE:
                break
                
            # Быстрое квантование байтов (0..255 -> 32..63)
            processed_frame = bytearray(
                32 + max(0, min(31, int(round(b / 8.2258)))) 
                for b in chunk
            )
            
            f_out.write(processed_frame)
            total_pixels += len(processed_frame)

    frames_count = total_pixels // FRAME_SIZE
    print(f"Готово! Обработано пикселей: {total_pixels} (Кадров: {frames_count}).")

convert_video_gray_raw_to_bin("video_320x200_gray.raw", "video_320x200_gray.bin")
```

<br>
<video controls width="100%" muted playsinline preload="metadata">
    <source src="/Computer-Science-Bookshelf/img/riscv/video/video_320x200_gray.mp4" type="video/mp4">
    Ваш браузер не поддерживает видео.
</video>

**Вариант rgb222**

Подготовка видеоряда к разрешению 320x200 с форматом rgb24, длительностью 15 секунд.
```
ffmpeg -ss 00:00:00 -i TomAndJerry.mp4 -t 15 -vf "scale=320:200,fps=15" -pix_fmt rgb24 -f rawvideo video_320x200_rgb24.raw
```

Файл img/riscv/video/convert_video_rgb222.py для конвертации данных 
```py
import math

def convert_video_raw_to_rgb222_bin(raw_filename, bin_filename):
    # Кадр raw24: 320 x 200 пикселей * 3 байта (R, G, B) = 192 000 байт
    RAW_FRAME_SIZE = 320 * 200 * 3 

    with open(raw_filename, "rb") as f_in, open(bin_filename, "wb") as f_out:
        total_pixels = 0
        
        while frame_bytes := f_in.read(RAW_FRAME_SIZE):
            # Пропускаем неполный кадр, если файл закончился раньше времени
            if len(frame_bytes) < RAW_FRAME_SIZE:
                break

            # Создаем буфер под сжатый кадр (64 000 байт)
            out_frame = bytearray(len(frame_bytes) // 3)
            
            # Быстрый шаг по 3 байта (R, G, B)
            for i in range(0, len(frame_bytes), 3):
                r = frame_bytes[i]
                g = frame_bytes[i + 1]
                b = frame_bytes[i + 2]
                
                # RGB222 с офсетом +64
                color_index = 64 + ((r >> 6) << 4) + ((g >> 6) << 2) + (b >> 6)
                out_frame[i // 3] = color_index
            
            f_out.write(out_frame)
            total_pixels += len(out_frame)

    frames_count = total_pixels // (320 * 200)
    print(f"Готово! Обработано пикселей: {total_pixels} (Кадров: {frames_count}).")

convert_video_raw_to_rgb222_bin("video_320x200_rgb24.raw", "video_320x200_rgb222.bin")
```

<br>
<video controls width="100%" muted playsinline preload="metadata">
    <source src="/Computer-Science-Bookshelf/img/riscv/video/video_320x200_rgb222.mp4" type="video/mp4">
    Ваш браузер не поддерживает видео.
</video>


> Вот только наш процессор тут ни при чем: это просто демонстрация слабой стороны процессора, когда он "захлебывается" копированием большого обьема данных из памяти. А на отрисовку фрактала (Множество Мандельброта) уйдет аж 5-7 минут.

 
 
## Схемы, код и данные

* [Схемы](/Computer-Science-Bookshelf/RISC-V/program/ProcessorRiscvDig.zip)
* [Ассемблер и код программ](/Computer-Science-Bookshelf/RISC-V/program/ProcessorCode.zip)
* [Данные](/Computer-Science-Bookshelf/RISC-V/program/DATA.zip)

















---

<script>
document.addEventListener("DOMContentLoaded", function () {
  document.querySelectorAll('.abraetablemini.ace_editor').forEach(el => {
    const editor = ace.edit(el);
    const scroller = editor.container.querySelector('.ace_scroller');
    scroller.addEventListener('wheel', function(e) {
      e.preventDefault();
      scroller.scrollTop += e.deltaY;
    });
  });
});
</script>

<style>
table {
  margin: 0px !important;  
  border-collapse: collapse;
}
.abraetablemini.ace_editor {
  height: 300px !important;
}
.abraetablemini  {
    border: 2px solid purple;  
    border-radius: 8px;      
    padding: 10px;          
    background-color: #f5f5f5;  
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);  
    border-color: #8a2be2; 
}
</style> 
