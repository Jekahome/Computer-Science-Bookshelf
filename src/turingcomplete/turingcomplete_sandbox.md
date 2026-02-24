# Sandbox

## Игра "Half a worm"

> Ради применения LEG процессора - разработаем игру.
>
> Червячок имеет внушительный размер - аж 4. Расти червячок не может, но и есть не просит, при этом имеет огромные просторы для странствий. 256 байт памяти программы с 4-байтными инструкциями не хватило для полноценной реализации логики игры. Интерес не в самой игре, а в ограничениях при ее создании. Теперь можно закрыть тему 8-битного процессора и ознакомливаться с более мощными архитектурами.

Компонент `IO > DIS > Console`:
* Экран 80x24 (80 width, 24 height) 1920 пикселей ASCII символов. 
* Подключается к RAM через Link Components. 

8-ми битный процессор может адресовать ячейки только 0-255, но Console 80x24 имеет 1920 ячеек. 

Схема "bank + адрес в bank" позволяет имитировать полное адресное пространство.
* linear - адрес на экране
* bank - номер окна RAM
* addr - 8‑битный адрес в окне
* vram[bank * 256 + addr] - физический адрес в 8‑битной RAM

```rust,editable
fn main() {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 24;
    const BANK_SIZE: usize = 256;
    const BANK_COUNT: usize = 8;

    let mut vram = [b'_'; BANK_SIZE * BANK_COUNT];

    let x = 10;
    let y = 5;

    let linear = y * WIDTH + x;
    let bank = linear >> 8;
    let addr = linear & 0xFF;

    vram[bank * BANK_SIZE + addr] = b'A';

    println!(
        "coord=({},{}), linear={}, bank={}, addr={}, value={}",
        x, y, linear, bank, addr, vram[bank * BANK_SIZE + addr] as char
    );
}
```

Компонет `IO > SAN > Keyboard`:
* Для запоминания ввода, добавим регистр
* Подключается к логике Input
    * 69 left
    * 70 up
    * 72 down
    * 71 right
  

<details>
<summary>Прототип:</summary>

```rust,editable,abraetable

#![allow(dead_code)]

#[derive(Clone, Debug, Copy)]
enum D{
    R,/*RIGHT*/
    L,/*LEFT*/
    U,/*UP*/
    D/*DOWN*/
}
 
fn next_coords(
    x: usize,
    y: usize,
    mut direct: [D; 4],
    next_step_head: D
) -> Vec<(usize, usize)> {
 
    let mut result = Vec::new();
 
    /*direct[0] = direct[1];
    direct[1] = direct[2];
    direct[2] = direct[3];
    direct[3] = next_step_head;
    */
    //--------------------------
    let r4 = direct[3];     // old ram[3]
    let r5 = direct[2];     // old ram[2]
    direct[2] = r4;         // ram[2] = old ram[3]
    let r4 = direct[1];     // old ram[1]
    direct[1] = r5;         // ram[1] = old ram[2]
    direct[0] = r4;         // ram[0] = old ram[1]
    direct[3] = next_step_head; 
    
    // восстанавливаем тело по направлениям от старой головы к хвосту
    let (mut cur_x, mut cur_y) = (x, y);
    for i in (0..=2).rev() {  
        match direct[i] {
            D::R => cur_x -= 1,
            D::L => cur_x += 1,
            D::U => cur_y += 1,
            D::D => cur_y -= 1,
        }
        
        result.insert(0, (cur_x, cur_y));
    }
    
    
    // новая позиция головы
    let ( cur_x, cur_y) = match next_step_head {
        D::R => (x + 1, y),
        D::L => (x - 1, y),
        D::U => (x, y - 1),
        D::D => (x, y + 1),
    };
    result.push((x, y)); 
    result.push((cur_x, cur_y)); // голова в конец

    result
}

fn main() {
     
    let mut grid = NotebookGrid::new(6, 6);
  
    let x = 2;
    let y = 4;
    let head = {
            (x, y)
    };
    let body_1 = (1, 4);
    let body_2 = (1, 3);
    let body_3 = (1, 2);
    let tail = (0, 2);
     
    // Массив координат змейки
    let snake = vec![
        tail, body_3, body_2, body_1, head
    ];
    
    grid.draw_snake(&snake);
    grid.display();
    
    //==========================================
    // prev step
    //-------------------body_1 body_2 body_3 head
    let direct:[D; 4] = [D::R,  D::D,  D::D,  D::R];  
       
    let next_step_head = D::R;
    
    let snake = next_coords(x,y, direct  ,next_step_head);
  
    grid.draw_snake(&snake);
    grid.display();
}





# struct NotebookGrid {
#     width: usize,
#     height: usize,
#     cells: Vec<Vec<bool>>,
# }
#
# impl NotebookGrid {
#     fn new(width: usize, height: usize) -> Self {
#         NotebookGrid {
#             width,
#             height,
#             cells: vec![vec![false; width]; height],
#         }
#     }
#     
#     fn fill_cell(&mut self, x: usize, y: usize) {
#         if x < self.width && y < self.height {
#             self.cells[y][x] = true;
#         }
#     }
#     
#     fn clear_cell(&mut self, x: usize, y: usize) {
#         if x < self.width && y < self.height {
#             self.cells[y][x] = false;
#         }
#     }
#     
#     fn fill_rect(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
#         for y in y1..=y2 {
#             for x in x1..=x2 {
#                 self.fill_cell(x, y);
#             }
#         }
#     }
#     
#     // Новый метод для отрисовки змейки
#     fn draw_snake(&mut self, snake: &[(usize, usize)]) {
#         // Очищаем поле
#         for y in 0..self.height {
#             for x in 0..self.width {
#                 self.cells[y][x] = false;
#             }
#         }
#         
#         // Рисуем змейку
#         for &(x, y) in snake {
#             self.fill_cell(x, y);
#         }
#     }
#     
#     fn display(&self) {
#         // Верхний колонтитул с цифрами
#         print!(r"y\x");
#         for x in 0..self.width {
#             if x < 10 {
#                 print!(" {} ", x);
#             } else {
#                 print!("{} ", x);
#             }
#         }
#         println!();
#         
#         // Верхняя граница
#         print!("   ┌");
#         for x in 0..self.width {
#             print!("──");
#             if x < self.width - 1 {
#                 print!("┬");
#             }
#         }
#         println!("┐");
#         
#         // Основное поле
#         for y in 0..self.height {
#             // Номер строки
#             if y < 10 {
#                 print!(" {} │", y);
#             } else {
#                 print!("{} │", y);
#             }
#             
#             // Клетки
#             for x in 0..self.width {
#                 if self.cells[y][x] {
#                     print!("█");
#                     print!("█");
#                 } else {
#                     print!("·");
#                     print!("·");
#                 }
#                 
#                 if x < self.width - 1 {
#                     print!("│");
#                 }
#             }
#             println!("│");
#             
#             // Горизонтальные линии между строками
#             if y < self.height - 1 {
#                 print!("   ├");
#                 for x in 0..self.width {
#                     print!("──");
#                     if x < self.width - 1 {
#                         print!("┼");
#                     }
#                 }
#                 println!("┤");
#             }
#         }
#         
#         // Нижняя граница
#         print!("   └");
#         for x in 0..self.width {
#             print!("──");
#             if x < self.width - 1 {
#                 print!("┴");
#             }
#         }
#         println!("┘");
#     }
# }
```

</details>

<br>
  
Для проверок переполнения оси X не хватило места в программе (248 байт), так же и для логики snake (увеличения тела, еды...)

 
<br>

```rust,editable,abraetable
use legassembly::assembly;
fn main(){
  let output_debug = true;
  static INPUT: &str = "
## io output Draw ASCII 48 snake или 0-пусто
## r3 в роли приемника INPUT
## r2 Head X
## r1 Head Y

## Все тело snake храним в RAM [0:tail,1:body,2:body,3:head]
## Fixed snake length - 4 
## Храним только историю направление движения одного шага. 
## RIGHT - 0
## LEFT  - 1
## UP    - 2
## DOWN  - 3
## start position X:3 Y:0
MOV 3, r2 ## Head X

read_input:
    MOV io, r3
   
    ## сдвигаем направления body
    MOV 3,r0
    MOV ram,r4      

    MOV 2,r0
    MOV ram,r5      
    MOV r4,ram       

    MOV 1,r0
    MOV ram,r4
    MOV r5,ram       

    MOV 0,r0
    MOV r4,ram  
  
    ## сохраним текущее положение
    MOV r1, r4 # copy r1 to r4
    MOV r2, r5 # copy r2 to r5

## высчитываем XY для удаления tail ------------
    ## while восстанавливаем тело пропуская head
    MOV 2, r0 ## index array body
    while_find_tail:
        CJE ram, 0, set_body_right
        CJE ram, 1, set_body_left
        CJE ram, 2, set_body_up
        # CJE ram, 3, set_body_down ## default

        # set_body_down:
            CJE r1, 0, pre_reset_y_up
            SUB r1, 1, r1 ## Head Y
            JMP draw_body
         
        set_body_right:
            SUB r2, 1, r2 ## Head X
            JMP draw_body

        set_body_left:
            ADD 1, r2, r2 ## Head X
            JMP draw_body

        set_body_up:
            CJG r1, 23, pre_reset_y_down
            ADD 1, r1, r1 ## Head Y
            JMP draw_body

        pre_reset_y_up:
            MOV 23, r1   
            JMP draw_body

        pre_reset_y_down:
            MOV 0, r1   
            ## JMP draw_body

        draw_body:
            CJE r0, 0, remove_tail
            SUB r0, 1, r0
            JMP while_find_tail

    remove_tail:
        MOV 0 io  ## remove tail
## --------------------------------------
## restor r1,r2
MOV r4, r1
MOV r5, r2

MOV 3, r0 ## для следующего результата по if

CJE r3, 69, set_head_left
CJE r3, 70, set_head_up
CJE r3, 72, set_head_down
## CJE r3, 71, set_head_right
JMP set_head_right ## default set_head_right

set_head_down:
    MOV 3, ram ## next_step_head
    CJG r1, 23, reset_y_up
    ADD 1, r1, r1 ## Head Y
    JMP body_pos
set_head_up:
    MOV 2, ram ## next_step_head
    CJE r1, 0, reset_y_down
    SUB r1, 1, r1 ## Head Y
    JMP body_pos
set_head_left:
    MOV 1, ram ## next_step_head
    ## тут должна быть проверка переполнения
    SUB r2, 1, r2 ## Head X
    JMP body_pos
set_head_right:
    MOV 0, ram ## next_step_head
    ## тут должна быть проверка переполнения
    ADD 1, r2, r2 ## Head X
    JMP body_pos
reset_y_up:
    MOV 0, r1   
    JMP body_pos
reset_y_down:
    MOV 24, r1   
    ## JMP body_pos
body_pos:
    MOV 48 io  ## [X:r2; Y:r1]=48 to output
    JMP read_input
";
 
  assembly(INPUT, output_debug);
}

{{#include ../../src/legassembly.rs}}

```

---

<br>
<video controls muted width="100%" playsinline preload="metadata" onloadedmetadata="this.playbackRate = 1.0">
    <source src="/Computer-Science-Bookshelf/img/tc/snake.mp4" type="video/mp4">
    Ваш браузер не поддерживает видео. 
</video>
 
---

<script>
document.addEventListener("DOMContentLoaded", function () {
  document.querySelectorAll('.abraetable.ace_editor').forEach(el => {
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
.abraetable.ace_editor {
  height: 1000px !important;
}
.abraetable  {
    border: 2px solid purple;  
    border-radius: 8px;      
    padding: 10px;          
    background-color: #f5f5f5;  
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);  
    border-color: #8a2be2; 
}
</style> 

