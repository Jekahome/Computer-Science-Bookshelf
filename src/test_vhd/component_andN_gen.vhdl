library ieee;
use ieee.std_logic_1164.all;

entity andn is
  generic (
    width : integer := 8
  );
  port (
    a : in    std_logic_vector(width - 1 downto 0);
    y : out   std_logic
  );
end entity andn;

architecture synth of andn is
  signal x : std_logic_vector(width - 1 downto 0);
begin
  x(0) <= a(0);
  gen : for i in 1 to width - 1 generate
    x(i) <= a(i) and x(i - 1);
  end generate gen;
    y <= x(width - 1);
end architecture synth;
