library IEEE; 
use IEEE.STD_LOGIC_1164.all;

entity component_priority_casez is
    port(
        a: in STD_LOGIC_VECTOR(3 downto 0);
        y: out STD_LOGIC_VECTOR(3 downto 0)
    );
end entity component_priority_casez; 

architecture dontcare of component_priority_casez is
begin
    process(all) begin
        case? a is
            when "1---" => y <= "1000";
            when "01--" => y <= "0100";
            when "001-" => y <= "0010";
            when "0001" => y <= "0001";
            when others => y <= "0000";
        end case?; 
    end process;
end architecture dontcare;