library ieee;

use ieee.std_logic_1164.all;

entity sram_cell is
  port (
    wl     : in  std_logic; -- Линия выбора ячейки (Word Line)
    we     : in  std_logic; -- Разрешение записи (Write Enable): '1' - запись, '0' - чтение
    din    : in  std_logic; -- Вход данных (Bit Line для записи)
    din_n  : in  std_logic; -- Инверсный вход данных (Inverse Bit Line)
    dout   : out std_logic; -- Выход данных (Bit Line для чтения)
    dout_n : out std_logic  -- Инверсный выход данных (Inverse Bit Line)
  );
end entity sram_cell;

architecture structural of sram_cell is
  signal q   : std_logic := '0'; -- Внутреннее состояние ячейки
  signal q_n : std_logic := '1'; -- Инверсное внутреннее состояние
begin

  -- Логика хранения и записи
  process(wl, we, din, din_n, q, q_n)
  begin
    if (wl = '1' and we = '1') then
      -- Если ячейка выбрана и включена запись — сохраняем то, что пришло на входы
      q   <= din;
      q_n <= din_n;
    else
      -- В режиме чтения или хранения инверторы держат друг друга по кругу
      q   <= not q_n;
      q_n <= not q;
    end if;
  end process;

  -- Выходы выдают данные наружу, только если линия слова wl = '1'
  dout   <= q   when wl = '1' else 'Z';
  dout_n <= q_n when wl = '1' else 'Z';

end architecture structural;