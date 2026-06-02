library IEEE; 
use IEEE.STD_LOGIC_1164.all;

entity component_priorityckt is
port(
    a: in STD_LOGIC_VECTOR(3 downto 0);
    y: out STD_LOGIC_VECTOR(3 downto 0)
    );
end;

architecture synth of component_priorityckt is
begin
    process(all) begin
        if a(3) = '1' then 
            y <= "1000";
        elsif a(2) = '1' then 
            y <= "0100";
        elsif a(1) = '1' then 
            y <= "0010";
        elsif a(0) = '1' then 
            y <= "0001";
        else 
            y <= "0000";
        end if;
    end process;
end;