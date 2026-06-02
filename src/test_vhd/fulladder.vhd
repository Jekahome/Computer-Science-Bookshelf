library IEEE;
use IEEE.STD_LOGIC_1164.all;
entity fulladder is
  port (
    a, b, cin : in std_logic;
    s, cout   : out std_logic
  );
end;

architecture synth of fulladder is
  signal p, g : std_logic; -- internal variables
begin
  p    <= a xor b;
  g    <= a and b;
  s    <= p xor cin;
  cout <= g or (p and cin); -- parentheses for operator precedence
end;