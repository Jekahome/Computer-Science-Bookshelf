library IEEE; use IEEE.STD_LOGIC_1164.all;

entity component_flop is
    port(clk: in STD_LOGIC;
        d: in STD_LOGIC_VECTOR(3 downto 0);
        q: out STD_LOGIC_VECTOR(3 downto 0)
    );
end;


architecture synth of component_flop is
begin
    process(clk) begin
        if rising_edge(clk) then
            q <= d;
        end if;
    end process;
end;