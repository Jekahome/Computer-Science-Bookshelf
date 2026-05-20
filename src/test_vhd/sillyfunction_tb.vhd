
library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity sillyfunction_tb is
end;

architecture bench of sillyfunction_tb is
  -- Clock period
  constant clk_period : time := 10 ns;
  -- Generics
  -- Ports
  signal a : std_logic;
  signal b : std_logic;
  signal c : std_logic;
  signal y : std_logic;
begin

  -- Подключаем вашу функцию sillyfunction из файла test.vhd
  sillyfunction_inst : entity work.sillyfunction
  port map (
    a => a,
    b => b,
    c => c,
    y => y
  );
-- clk <= not clk after clk_period/2;

-- Сам тест: меняем входы каждые 10 наносекунд
  stimulus : process
  begin
    -- Проверка 1: все нули (y должен стать '1')
    a <= '0'; b <= '0'; c <= '0'; wait for clk_period;
    
    -- Проверка 2: только c='1' (y должен стать '0')
    a <= '0'; b <= '0'; c <= '1'; wait for clk_period;
    
    -- Проверка 3: только a='1' (y должен стать '1')
    a <= '1'; b <= '0'; c <= '0'; wait for clk_period;
    
    -- Проверка 4: a='1' и c='1' (y должен стать '1')
    a <= '1'; b <= '0'; c <= '1'; wait for clk_period;

    -- Конец теста
    wait;
  end process;

end;