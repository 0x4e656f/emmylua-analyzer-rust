#[cfg(test)]
mod tests {
    use crate::handlers::test_lib::ProviderVirtualWorkspace;

    #[test]
    fn test_basic_definition() {
        let mut ws = ProviderVirtualWorkspace::new();
        ws.check_definition(
            r#"
                ---@generic T: string
                ---@param name `T`
                ---@return T
                local function new(name)
                    return name
                end

                ---@class Ability

                local a = new("<??>Ability")
            "#,
        );
    }

    #[test]
    fn test_table_field_definition_1() {
        let mut ws = ProviderVirtualWorkspace::new();
        ws.check_definition(
            r#"
                ---@class T
                ---@field func fun(self:string) 注释注释

                ---@type T
                local t = {
                    f<??>unc = function(self)
                    end
                }
            "#,
        );
    }

    #[test]
    fn test_table_field_definition_2() {
        let mut ws = ProviderVirtualWorkspace::new();
        ws.check_definition(
            r#"
                ---@class T
                ---@field func fun(self: T) 注释注释

                ---@type T
                local t = {
                    func = function(self)
                    end,
                    a = 1,
                }

                t:func<??>()
            "#,
        );
    }
}
