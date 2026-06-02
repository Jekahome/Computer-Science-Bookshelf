library IEEE; use IEEE.STD_LOGIC_1164.all;

entity component_flop_sync_reset is
    port(
        clk, reset: in STD_LOGIC;
        d: in STD_LOGIC_VECTOR(3 downto 0);
        q: out STD_LOGIC_VECTOR(3 downto 0)
    );
end;

architecture synchronous of component_flop_sync_reset is
begin
    process(clk) begin
        if rising_edge(clk) then
            if reset = '1' then q <= "0000";
                else q <= d;
            end if;
        end if;
    end process;
end;