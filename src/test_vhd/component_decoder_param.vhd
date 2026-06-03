library ieee;
  use ieee.std_logic_1164.all;
  use ieee. numeric_std_unsigned.all;

entity decoder is
  generic (
    n : integer := 3
  );
  port (
    a : in    std_logic_vector(N-1 downto 0);
    y : out   std_logic_vector(2 ** n - 1 downto 0)
  );
end entity decoder;

architecture synth of decoder is

begin
  process (all) is
  begin
    y                <= (others => '0');
    y(TO_INTEGER(a)) <= '1';
  end process;
end architecture synth;
