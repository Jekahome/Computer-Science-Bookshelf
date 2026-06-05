library ieee;
use ieee.std_logic_1164.all;

entity comparators is
  generic (
    n : integer := 8
  );
  port (
    a   : in    std_logic_vector(n - 1 downto 0);
    b   : in    std_logic_vector(n - 1 downto 0);
    eq  : out   std_logic;
    neq : out   std_logic;
    lt  : out   std_logic;
    lte : out   std_logic;
    gt  : out   std_logic;
    gte : out   std_logic
  );
end entity comparators;

architecture synth of comparators is
begin
  eq  <= '1' when (a = b) else
         '0';
  neq <= '1' when (a /= b) else
         '0';
  lt  <= '1' when (a < b) else
         '0';
  lte <= '1' when (a <= b) else
         '0';
  gt  <= '1' when (a > b) else
         '0';
  gte <= '1' when (a >= b) else
         '0';
end architecture synth;
