library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity ram_array is
  port (
    clk  : in    std_logic;
    we   : in    std_logic;
    adr  : in    std_logic_vector(5 downto 0);
    din  : in    std_logic_vector(31 downto 0);
    dout : out   std_logic_vector(31 downto 0)
  );
end entity ram_array;

architecture synth of ram_array is
  type mem_array is array (63 downto 0) of std_logic_vector(31 downto 0);
  signal mem : mem_array;
begin

  process (clk) is
  begin
    if rising_edge(clk) then
      if we = '1' then
        mem(to_integer(unsigned(adr))) <= din;
      end if;
    end if;
  end process;

  dout <= mem(to_integer(unsigned(adr)));

end architecture synth;