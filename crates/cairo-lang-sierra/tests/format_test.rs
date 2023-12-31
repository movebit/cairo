use indoc::indoc;
use test_log::test;

// Testing by parsing code and printing its display, making sure we get back the formatted code.
#[test]
fn format_test() {
    let parser = cairo_lang_sierra::ProgramParser::new();
    assert_eq!(
        parser
            .parse(indoc! {"
                // Some comment.
                type ConcreteTypeId =  TypeId; // Other comment.
                type ConcreteTypeId  = TypeId<arg>;
                type  ConcreteTypeId = TypeId<arg1, 4>;
                type [123] = TypeId<[12],  4>;
                type [4]= Enum<ut@core::option ::Option:: <core::felt252>, [3],[2]>;
                type Complex = Struct<ut@Complex:: <core::felt252, @core::felt252>,[8],[9]>;
                libfunc CalleeId = LibfuncId ;
                // Additional comment.
                libfunc OtherCalleeId = LibfuncId <arg, 4>;
                libfunc [5642] = LibfuncId<[22 ], 4>;
                libfunc CallFunction = Call<user@Function>;
                libfunc LibDependent = LibDependent<lib@[124]>;
                callee() -> ();
                callee(arg1) -> (res1);
                callee( arg1, arg2) -> ( res1, res2);
                callee() { 5( ) };
                callee(arg1 , arg2) { fallthrough() 7(res1 ) 5(res1, res2) };
                [12345]([12]) { 2([37]) fallthrough() };
                return();
                return ( r);
                return(r1 , r2);
                return ([1], [45], [0]);

                Name@5() -> ();
                Other@3([5]: T1) -> (T2);
                [343]@3([5]: [6343]) -> ([341]);
            "},)
            .map(|p| p.to_string()),
        Ok(indoc! {"
            type ConcreteTypeId = TypeId;
            type ConcreteTypeId = TypeId<arg>;
            type ConcreteTypeId = TypeId<arg1, 4>;
            type [123] = TypeId<[12], 4>;
            type [4] = Enum<ut@core::option::Option::<core::felt252>, [3], [2]>;
            type Complex = Struct<ut@Complex::<core::felt252, @core::felt252>, [8], [9]>;

            libfunc CalleeId = LibfuncId;
            libfunc OtherCalleeId = LibfuncId<arg, 4>;
            libfunc [5642] = LibfuncId<[22], 4>;
            libfunc CallFunction = Call<user@Function>;
            libfunc LibDependent = LibDependent<lib@[124]>;

            callee() -> (); // 0
            callee(arg1) -> (res1); // 1
            callee(arg1, arg2) -> (res1, res2); // 2
            callee() { 5() }; // 3
            callee(arg1, arg2) { fallthrough() 7(res1) 5(res1, res2) }; // 4
            [12345]([12]) { 2([37]) fallthrough() }; // 5
            return(); // 6
            return(r); // 7
            return(r1, r2); // 8
            return([1], [45], [0]); // 9

            Name@5() -> ();
            Other@3([5]: T1) -> (T2);
            [343]@3([5]: [6343]) -> ([341]);
        "}
        .to_string())
    );
}
