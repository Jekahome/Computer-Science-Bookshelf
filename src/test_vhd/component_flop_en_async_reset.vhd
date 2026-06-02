library ieee;

use ieee.std_logic_1164.all;

entity component_flop_en_async_reset is
  port (
    clk   : in    std_logic;
    reset : in    std_logic;
    en    : in    std_logic;
    d     : in    std_logic_vector(3 downto 0);
    q     : out   std_logic_vector(3 downto 0)
  );
end entity component_flop_en_async_reset;

architecture asynchronous of component_flop_en_async_reset is

-- асинхронный сброс

begin

  process (clk, reset) is
  begin

    if (reset = '1') then
      q <= "0000";
    elsif rising_edge(clk) then
      if (en = '1') then
        q <= d;
      end if;
    end if;

  end process;

end architecture asynchronous;