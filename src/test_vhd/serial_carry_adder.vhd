library ieee;
  use ieee.std_logic_1164.all;
  use ieee.numeric_std_unsigned.all;

entity adder is
  generic (
    n : integer := 8
  );
  port (
    a    : in    std_logic_vector(n - 1 downto 0);
    b    : in    std_logic_vector(n - 1 downto 0);
    cin  : in    std_logic;
    s    : out   std_logic_vector(n - 1 downto 0);
    cout : out   std_logic
  );
end entity adder;

architecture synth of adder is
  signal result : std_logic_vector(n downto 0);
begin
  result <= ("0" & a) + ("0" & b) + cin;
  s      <= result(n - 1 downto 0);
  cout   <= result(n);
end architecture synth;
