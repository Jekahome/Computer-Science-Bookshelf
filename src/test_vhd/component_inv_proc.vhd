library ieee;
  use ieee.std_logic_1164.all;

entity component_inv_proc is
  port (
    a : in    std_logic_vector(3 downto 0);
    y : out   std_logic_vector(3 downto 0)
  );
end entity component_inv_proc;

architecture proc of component_inv_proc is

begin

  process (all) is
  begin

    y <= not a;

  end process;

end architecture proc;
