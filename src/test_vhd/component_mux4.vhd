library IEEE;
use IEEE.STD_LOGIC_1164.all;
 

entity component_mux4 is
  port (
    d0, d1, d2, d3 : in std_logic_vector(3 downto 0);
    s : in std_logic_vector(1 downto 0);
    y : out std_logic_vector(3 downto 0)
    );
end;

architecture struct of component_mux4 is
  component mux2
    port (
      d0, d1 : in std_logic_vector(3 downto 0);
      s  : in std_logic;
      y  : out std_logic_vector(3 downto 0));
  end component;

  signal low, high : std_logic_vector(3 downto 0);

begin
  lowmux:  mux2 port map(d0, d1, s(0), low);
  highmux:  mux2 port map(d2, d3, s(0), high);
  finalmux:  mux2 port map(low, high, s(1), y);
end;