// Copyright 2024 Ryan McKenzie
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Flags;
use memchr::memmem;

fn do_test(mangled_name: &str, demangled_name: &str, partial_match: bool, flags: Flags) {
    let result = crate::demangle(mangled_name.into(), flags);
    match result {
        Ok(haystack) => {
            let matched = if partial_match {
                // this is how llvm checks their tests
                memmem::find(&haystack.as_bytes(), demangled_name.as_bytes()).is_some()
            } else {
                haystack == demangled_name
            };
            if !matched {
                assert!(
                    false,
                    "'{mangled_name}' <-- mangled string\n{flags:?} <-- flags\n'{demangled_name}' <-- expected\n'{haystack}' <-- actual",
                );
            }
        }
        Err(err) => assert!(
            false,
            "'{mangled_name}' <-- mangled string\n{err:?} <-- error",
        ),
    }
}

fn test(mangled_name: &str, demangled_name: &str) {
    do_test(mangled_name, demangled_name, true, Flags::default())
}

#[test]
fn test_invalid_manglings() {
    let test_invalid = |mangled_name: &str| {
        let result = crate::demangle(mangled_name, Flags::default());
        match result {
            Err(_) => (),
            Ok(demangled_name) => {
                assert!(
                    false,
                    "'{mangled_name}' <-- mangled string\n'Err(_)' <-- expected\n'{demangled_name}' <-- actual",
                );
            }
        }
    };

    test_invalid("?ff@@$$J0YAXAU?$AS_@$0A@PEAU?$AS_@$0A@H@__clang@@@__clang@@@Z");
    test_invalid("?f0@@YAXPEU?$AS_@$00$$CAD@__clang@@@Z");
    test_invalid("?@@8");
    test_invalid("??");
    test_invalid("??0@");
    test_invalid("? @@   YC@");
    test_invalid("??B@$$J0");
    test_invalid("??B@4");
    test_invalid("?A?@?@???B@4D");
    test_invalid("?A?@?@???B@4DD");
    test_invalid("??$A@P15@");
    test_invalid("??$A@P");
    test_invalid("?A@@");
    test_invalid("?A@@P");
    test_invalid("?A@@4PQA@@");
    test_invalid("??__E");
    test_invalid("??__E@@");
    test_invalid("??__E?Foo@@0HA@@");
    test_invalid("??__E?i@C@@0HA@");
    test_invalid("??__E?Foo@@YAXXZ");
    test_invalid("?foo@@YAH0@Z");
    test_invalid("?foo@@YAHH");
    test_invalid("??8@8");
    test_invalid("?B@?$?K$H?");
    test_invalid("??C@$");
    test_invalid("?x@@3PAW");
    test_invalid("??}");
    test_invalid("?foo@?$?_");
    test_invalid("??_R4");
    test_invalid("??_R4foo@@");
    test_invalid("?foo@?$?BH@@QAEHXZ");
    test_invalid("?foo@?$?0H@");
    test_invalid("??_C@_0A@01234567@a");
    test_invalid("??_C@_1A@01234567@a");
    test_invalid("??_C@_0301234567@a");
    test_invalid("??_C@_1301234567@a");
    test_invalid("??_C@_0601234567@abcdefghijklmnopqrtsuvwxyzABCDEFGHIJKLMNOPQRTSUVWXYZabcdefghijklmnopqrtsuvwxyzABCDEFGHIJKLMNOPQRTSUVWXYZabcdefghijklmnopqrtsuvwxyz");
    test_invalid("??_C@_12@?z");
    test_invalid("??$foo@$1??_C@_02PCEFGMJL@hi?$AA@@");
    test_invalid("??_C@");
    test_invalid("??_C@_");
    test_invalid("??_C@_3");
    test_invalid("??_C@_01");
    test_invalid("??_C@_0101234567@");
    test_invalid("??_C@_0101234567@?");
    test_invalid("??_C@_0101234567@?$");
    test_invalid("??_C@_0101234567@?$za");
    test_invalid("??_C@_0101234567@?$az");
    test_invalid("??_C@_1201234567@a?$az");
    test_invalid("??@foo");
    test_invalid("?foo@@3YA@A");
    test_invalid("?foo@@3Y~01KA");
    test_invalid("?foo@@3Y0~1KA");
    test_invalid("?x@@3PEAY02$$CRHEA");
    test_invalid("?foo@@3_");
    test_invalid("?foo@@3_XA");
    test_invalid("?foo@@3Vbar");
    test_invalid("?foo@@3Vbar@");
    test_invalid("?foo@?A");
    test_invalid("?foo@?");
    test_invalid("?foo@??");
    test_invalid("?foo@?XX?");
    test_invalid("?foo@?A@?");
    test_invalid("?foo@?Q@?");
    test_invalid("?foo@?BQ@?");
    test_invalid("?foo@?0?");
    test_invalid("??_Sfoo@@1Abar@@");
    test_invalid("??_Bfoo@@1");
    test_invalid("??_R0");
    test_invalid("??_R0H");
    test_invalid("??_R0H@8foo");
    test_invalid("??_R1012?3foo@@");
    test_invalid("??_R2foo@@1");
    test_invalid("??_A");
    test_invalid("??_P");
    test_invalid(".?AUBase@@@8");
}

#[test]
fn test_arg_qualifiers() {
    test("?foo@@YAXI@Z", "void __cdecl foo(unsigned int)");
    test("?foo@@YAXN@Z  ", "void __cdecl foo(double)");
    test("?foo_pad@@YAXPAD@Z", "void __cdecl foo_pad(char *)");
    test("?foo_pad@@YAXPEAD@Z", "void __cdecl foo_pad(char *)");
    test("?foo_pbd@@YAXPBD@Z", "void __cdecl foo_pbd(char const *)");
    test("?foo_pbd@@YAXPEBD@Z", "void __cdecl foo_pbd(char const *)");
    test(
        "?foo_pcd@@YAXPCD@Z",
        "void __cdecl foo_pcd(char volatile *)",
    );
    test(
        "?foo_pcd@@YAXPECD@Z",
        "void __cdecl foo_pcd(char volatile *)",
    );
    test("?foo_qad@@YAXQAD@Z", "void __cdecl foo_qad(char *const)");
    test("?foo_qad@@YAXQEAD@Z", "void __cdecl foo_qad(char *const)");
    test("?foo_rad@@YAXRAD@Z", "void __cdecl foo_rad(char *volatile)");
    test(
        "?foo_rad@@YAXREAD@Z",
        "void __cdecl foo_rad(char *volatile)",
    );
    test(
        "?foo_sad@@YAXSAD@Z",
        "void __cdecl foo_sad(char *const volatile)",
    );
    test(
        "?foo_sad@@YAXSEAD@Z",
        "void __cdecl foo_sad(char *const volatile)",
    );
    test(
        "?foo_piad@@YAXPIAD@Z",
        "void __cdecl foo_piad(char *__restrict)",
    );
    test(
        "?foo_piad@@YAXPEIAD@Z",
        "void __cdecl foo_piad(char *__restrict)",
    );
    test(
        "?foo_qiad@@YAXQIAD@Z",
        "void __cdecl foo_qiad(char *const __restrict)",
    );
    test(
        "?foo_qiad@@YAXQEIAD@Z",
        "void __cdecl foo_qiad(char *const __restrict)",
    );
    test(
        "?foo_riad@@YAXRIAD@Z",
        "void __cdecl foo_riad(char *volatile __restrict)",
    );
    test(
        "?foo_riad@@YAXREIAD@Z",
        "void __cdecl foo_riad(char *volatile __restrict)",
    );
    test(
        "?foo_siad@@YAXSIAD@Z",
        "void __cdecl foo_siad(char *const volatile __restrict)",
    );
    test(
        "?foo_siad@@YAXSEIAD@Z",
        "void __cdecl foo_siad(char *const volatile __restrict)",
    );
    test("?foo_papad@@YAXPAPAD@Z", "void __cdecl foo_papad(char **)");
    test(
        "?foo_papad@@YAXPEAPEAD@Z",
        "void __cdecl foo_papad(char **)",
    );
    test(
        "?foo_papbd@@YAXPAPBD@Z",
        "void __cdecl foo_papbd(char const **)",
    );
    test(
        "?foo_papbd@@YAXPEAPEBD@Z",
        "void __cdecl foo_papbd(char const **)",
    );
    test(
        "?foo_papcd@@YAXPAPCD@Z",
        "void __cdecl foo_papcd(char volatile **)",
    );
    test(
        "?foo_papcd@@YAXPEAPECD@Z",
        "void __cdecl foo_papcd(char volatile **)",
    );
    test(
        "?foo_pbqad@@YAXPBQAD@Z",
        "void __cdecl foo_pbqad(char *const *)",
    );
    test(
        "?foo_pbqad@@YAXPEBQEAD@Z",
        "void __cdecl foo_pbqad(char *const *)",
    );
    test(
        "?foo_pcrad@@YAXPCRAD@Z",
        "void __cdecl foo_pcrad(char *volatile *)",
    );
    test(
        "?foo_pcrad@@YAXPECREAD@Z",
        "void __cdecl foo_pcrad(char *volatile *)",
    );
    test(
        "?foo_qapad@@YAXQAPAD@Z",
        "void __cdecl foo_qapad(char **const)",
    );
    test(
        "?foo_qapad@@YAXQEAPEAD@Z",
        "void __cdecl foo_qapad(char **const)",
    );
    test(
        "?foo_rapad@@YAXRAPAD@Z",
        "void __cdecl foo_rapad(char **volatile)",
    );
    test(
        "?foo_rapad@@YAXREAPEAD@Z",
        "void __cdecl foo_rapad(char **volatile)",
    );
    test(
        "?foo_pbqbd@@YAXPBQBD@Z",
        "void __cdecl foo_pbqbd(char const *const *)",
    );
    test(
        "?foo_pbqbd@@YAXPEBQEBD@Z",
        "void __cdecl foo_pbqbd(char const *const *)",
    );
    test(
        "?foo_pbqcd@@YAXPBQCD@Z",
        "void __cdecl foo_pbqcd(char volatile *const *)",
    );
    test(
        "?foo_pbqcd@@YAXPEBQECD@Z",
        "void __cdecl foo_pbqcd(char volatile *const *)",
    );
    test(
        "?foo_pcrbd@@YAXPCRBD@Z",
        "void __cdecl foo_pcrbd(char const *volatile *)",
    );
    test(
        "?foo_pcrbd@@YAXPECREBD@Z",
        "void __cdecl foo_pcrbd(char const *volatile *)",
    );
    test(
        "?foo_pcrcd@@YAXPCRCD@Z",
        "void __cdecl foo_pcrcd(char volatile *volatile *)",
    );
    test(
        "?foo_pcrcd@@YAXPECRECD@Z",
        "void __cdecl foo_pcrcd(char volatile *volatile *)",
    );
    test("?foo_aad@@YAXAAD@Z", "void __cdecl foo_aad(char &)");
    test("?foo_aad@@YAXAEAD@Z", "void __cdecl foo_aad(char &)");
    test("?foo_abd@@YAXABD@Z", "void __cdecl foo_abd(char const &)");
    test("?foo_abd@@YAXAEBD@Z", "void __cdecl foo_abd(char const &)");
    test("?foo_aapad@@YAXAAPAD@Z", "void __cdecl foo_aapad(char *&)");
    test(
        "?foo_aapad@@YAXAEAPEAD@Z",
        "void __cdecl foo_aapad(char *&)",
    );
    test(
        "?foo_aapbd@@YAXAAPBD@Z",
        "void __cdecl foo_aapbd(char const *&)",
    );
    test(
        "?foo_aapbd@@YAXAEAPEBD@Z",
        "void __cdecl foo_aapbd(char const *&)",
    );
    test(
        "?foo_abqad@@YAXABQAD@Z",
        "void __cdecl foo_abqad(char *const &)",
    );
    test(
        "?foo_abqad@@YAXAEBQEAD@Z",
        "void __cdecl foo_abqad(char *const &)",
    );
    test(
        "?foo_abqbd@@YAXABQBD@Z",
        "void __cdecl foo_abqbd(char const *const &)",
    );
    test(
        "?foo_abqbd@@YAXAEBQEBD@Z",
        "void __cdecl foo_abqbd(char const *const &)",
    );
    test(
        "?foo_aay144h@@YAXAAY144H@Z",
        "void __cdecl foo_aay144h(int (&)[5][5])",
    );
    test(
        "?foo_aay144h@@YAXAEAY144H@Z",
        "void __cdecl foo_aay144h(int (&)[5][5])",
    );
    test(
        "?foo_aay144cbh@@YAXAAY144$$CBH@Z",
        "void __cdecl foo_aay144cbh(int const (&)[5][5])",
    );
    test(
        "?foo_aay144cbh@@YAXAEAY144$$CBH@Z",
        "void __cdecl foo_aay144cbh(int const (&)[5][5])",
    );
    test(
        "?foo_qay144h@@YAX$$QAY144H@Z",
        "void __cdecl foo_qay144h(int (&&)[5][5])",
    );
    test(
        "?foo_qay144h@@YAX$$QEAY144H@Z",
        "void __cdecl foo_qay144h(int (&&)[5][5])",
    );
    test(
        "?foo_qay144cbh@@YAX$$QAY144$$CBH@Z",
        "void __cdecl foo_qay144cbh(int const (&&)[5][5])",
    );
    test(
        "?foo_qay144cbh@@YAX$$QEAY144$$CBH@Z",
        "void __cdecl foo_qay144cbh(int const (&&)[5][5])",
    );
    test(
        "?foo_p6ahxz@@YAXP6AHXZ@Z",
        "void __cdecl foo_p6ahxz(int (__cdecl *)(void))",
    );
    test(
        "?foo_p6ahxz@@YAXP6AHXZ@Z",
        "void __cdecl foo_p6ahxz(int (__cdecl *)(void))",
    );
    test(
        "?foo_a6ahxz@@YAXA6AHXZ@Z",
        "void __cdecl foo_a6ahxz(int (__cdecl &)(void))",
    );
    test(
        "?foo_a6ahxz@@YAXA6AHXZ@Z",
        "void __cdecl foo_a6ahxz(int (__cdecl &)(void))",
    );
    test(
        "?foo_q6ahxz@@YAX$$Q6AHXZ@Z",
        "void __cdecl foo_q6ahxz(int (__cdecl &&)(void))",
    );
    test(
        "?foo_q6ahxz@@YAX$$Q6AHXZ@Z",
        "void __cdecl foo_q6ahxz(int (__cdecl &&)(void))",
    );
    test(
        "?foo_qay04h@@YAXQAY04H@Z",
        "void __cdecl foo_qay04h(int (*const)[5])",
    );
    test(
        "?foo_qay04h@@YAXQEAY04H@Z",
        "void __cdecl foo_qay04h(int (*const)[5])",
    );
    test(
        "?foo_qay04cbh@@YAXQAY04$$CBH@Z",
        "void __cdecl foo_qay04cbh(int const (*const)[5])",
    );
    test(
        "?foo_qay04cbh@@YAXQEAY04$$CBH@Z",
        "void __cdecl foo_qay04cbh(int const (*const)[5])",
    );
    test("?foo@@YAXPAY02N@Z", "void __cdecl foo(double (*)[3])");
    test("?foo@@YAXPEAY02N@Z", "void __cdecl foo(double (*)[3])");
    test("?foo@@YAXQAN@Z", "void __cdecl foo(double *const)");
    test("?foo@@YAXQEAN@Z", "void __cdecl foo(double *const)");
    test(
        "?foo_const@@YAXQBN@Z",
        "void __cdecl foo_const(double const *const)",
    );
    test(
        "?foo_const@@YAXQEBN@Z",
        "void __cdecl foo_const(double const *const)",
    );
    test(
        "?foo_volatile@@YAXQCN@Z",
        "void __cdecl foo_volatile(double volatile *const)",
    );
    test(
        "?foo_volatile@@YAXQECN@Z",
        "void __cdecl foo_volatile(double volatile *const)",
    );
    test(
        "?foo@@YAXPAY02NQBNN@Z",
        "void __cdecl foo(double (*)[3], double const *const, double)",
    );
    test(
        "?foo@@YAXPEAY02NQEBNN@Z",
        "void __cdecl foo(double (*)[3], double const *const, double)",
    );
    test(
        "?foo_fnptrconst@@YAXP6AXQAH@Z@Z",
        "void __cdecl foo_fnptrconst(void (__cdecl *)(int *const))",
    );
    test(
        "?foo_fnptrconst@@YAXP6AXQEAH@Z@Z",
        "void __cdecl foo_fnptrconst(void (__cdecl *)(int *const))",
    );
    test(
        "?foo_fnptrarray@@YAXP6AXQAH@Z@Z",
        "void __cdecl foo_fnptrarray(void (__cdecl *)(int *const))",
    );
    test(
        "?foo_fnptrarray@@YAXP6AXQEAH@Z@Z",
        "void __cdecl foo_fnptrarray(void (__cdecl *)(int *const))",
    );
    test("?foo_fnptrbackref1@@YAXP6AXQAH@Z1@Z", "void __cdecl foo_fnptrbackref1(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test("?foo_fnptrbackref1@@YAXP6AXQEAH@Z1@Z", "void __cdecl foo_fnptrbackref1(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test("?foo_fnptrbackref2@@YAXP6AXQAH@Z1@Z", "void __cdecl foo_fnptrbackref2(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test("?foo_fnptrbackref2@@YAXP6AXQEAH@Z1@Z", "void __cdecl foo_fnptrbackref2(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test("?foo_fnptrbackref3@@YAXP6AXQAH@Z1@Z", "void __cdecl foo_fnptrbackref3(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test("?foo_fnptrbackref3@@YAXP6AXQEAH@Z1@Z", "void __cdecl foo_fnptrbackref3(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test(
        "?foo_fnptrbackref4@@YAXP6AXPAH@Z1@Z",
        "void __cdecl foo_fnptrbackref4(void (__cdecl *)(int *), void (__cdecl *)(int *))",
    );
    test(
        "?foo_fnptrbackref4@@YAXP6AXPEAH@Z1@Z",
        "void __cdecl foo_fnptrbackref4(void (__cdecl *)(int *), void (__cdecl *)(int *))",
    );
    test(
        "?ret_fnptrarray@@YAP6AXQAH@ZXZ",
        "void (__cdecl * __cdecl ret_fnptrarray(void))(int *const)",
    );
    test(
        "?ret_fnptrarray@@YAP6AXQEAH@ZXZ",
        "void (__cdecl * __cdecl ret_fnptrarray(void))(int *const)",
    );
    test(
        "?mangle_no_backref0@@YAXQAHPAH@Z",
        "void __cdecl mangle_no_backref0(int *const, int *)",
    );
    test(
        "?mangle_no_backref0@@YAXQEAHPEAH@Z",
        "void __cdecl mangle_no_backref0(int *const, int *)",
    );
    test(
        "?mangle_no_backref1@@YAXQAHQAH@Z",
        "void __cdecl mangle_no_backref1(int *const, int *const)",
    );
    test(
        "?mangle_no_backref1@@YAXQEAHQEAH@Z",
        "void __cdecl mangle_no_backref1(int *const, int *const)",
    );
    test(
        "?mangle_no_backref2@@YAXP6AXXZP6AXXZ@Z",
        "void __cdecl mangle_no_backref2(void (__cdecl *)(void), void (__cdecl *)(void))",
    );
    test(
        "?mangle_no_backref2@@YAXP6AXXZP6AXXZ@Z",
        "void __cdecl mangle_no_backref2(void (__cdecl *)(void), void (__cdecl *)(void))",
    );
    test(
        "?mangle_yes_backref0@@YAXQAH0@Z",
        "void __cdecl mangle_yes_backref0(int *const, int *const)",
    );
    test(
        "?mangle_yes_backref0@@YAXQEAH0@Z",
        "void __cdecl mangle_yes_backref0(int *const, int *const)",
    );
    test(
        "?mangle_yes_backref1@@YAXQAH0@Z",
        "void __cdecl mangle_yes_backref1(int *const, int *const)",
    );
    test(
        "?mangle_yes_backref1@@YAXQEAH0@Z",
        "void __cdecl mangle_yes_backref1(int *const, int *const)",
    );
    test("?mangle_yes_backref2@@YAXQBQ6AXXZ0@Z", "void __cdecl mangle_yes_backref2(void (__cdecl *const *const)(void), void (__cdecl *const *const)(void))");
    test("?mangle_yes_backref2@@YAXQEBQ6AXXZ0@Z", "void __cdecl mangle_yes_backref2(void (__cdecl *const *const)(void), void (__cdecl *const *const)(void))");
    test("?mangle_yes_backref3@@YAXQAP6AXXZ0@Z", "void __cdecl mangle_yes_backref3(void (__cdecl **const)(void), void (__cdecl **const)(void))");
    test("?mangle_yes_backref3@@YAXQEAP6AXXZ0@Z", "void __cdecl mangle_yes_backref3(void (__cdecl **const)(void), void (__cdecl **const)(void))");
    test(
        "?mangle_yes_backref4@@YAXQIAH0@Z",
        "void __cdecl mangle_yes_backref4(int *const __restrict, int *const __restrict)",
    );
    test(
        "?mangle_yes_backref4@@YAXQEIAH0@Z",
        "void __cdecl mangle_yes_backref4(int *const __restrict, int *const __restrict)",
    );
    test(
        "?pr23325@@YAXQBUS@@0@Z",
        "void __cdecl pr23325(struct S const *const, struct S const *const)",
    );
    test(
        "?pr23325@@YAXQEBUS@@0@Z",
        "void __cdecl pr23325(struct S const *const, struct S const *const)",
    );
}

#[test]
fn test_auto_templates() {
    test(
        "??0?$AutoNTTPClass@$MPEAH1?i@@3HA@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<&int i>::AutoNTTPClass<&int i>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$1?i@@3HA@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<&int i>::AutoNTTPClass<&int i>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$MPEAH1?i@@3HA$MPEAH1?j@@3HA@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<&int i, &int j>::AutoNTTPClass<&int i, &int j>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$1?i@@3HA$1?j@@3HA@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<&int i, &int j>::AutoNTTPClass<&int i, &int j>(void)",
    );
    test("??0?$AutoNTTPClass@$MP6AHXZ1?Func@@YAHXZ@@QEAA@XZ", "public: __cdecl AutoNTTPClass<&int __cdecl Func(void)>::AutoNTTPClass<&int __cdecl Func(void)>(void)");
    test("??0?$AutoNTTPClass@$1?Func@@YAHXZ@@QEAA@XZ", "public: __cdecl AutoNTTPClass<&int __cdecl Func(void)>::AutoNTTPClass<&int __cdecl Func(void)>(void)");
    test("??0?$AutoNTTPClass@$MP6AHXZ1?Func@@YAHXZ$MP6AHXZ1?Func2@@YAHXZ@@QEAA@XZ", "public: __cdecl AutoNTTPClass<&int __cdecl Func(void), &int __cdecl Func2(void)>::AutoNTTPClass<&int __cdecl Func(void), &int __cdecl Func2(void)>(void)");
    test("??0?$AutoNTTPClass@$1?Func@@YAHXZ$1?Func2@@YAHXZ@@QEAA@XZ", "public: __cdecl AutoNTTPClass<&int __cdecl Func(void), &int __cdecl Func2(void)>::AutoNTTPClass<&int __cdecl Func(void), &int __cdecl Func2(void)>(void)");
    test(
        "??$AutoFunc@$MPEAH1?i@@3HA@@YA?A?<auto>@@XZ",
        "<auto> __cdecl AutoFunc<&int i>(void)",
    );
    test(
        "??$AutoFunc@$1?i@@3HA@@YA?A?<auto>@@XZ",
        "<auto> __cdecl AutoFunc<&int i>(void)",
    );
    test(
        "??$AutoFunc@$MP6AHXZ1?Func@@YAHXZ@@YA?A?<auto>@@XZ",
        "<auto> __cdecl AutoFunc<&int __cdecl Func(void)>(void)",
    );
    test(
        "??$AutoFunc@$1?Func@@YAHXZ@@YA?A?<auto>@@XZ",
        "<auto> __cdecl AutoFunc<&int __cdecl Func(void)>(void)",
    );
    test(
        "??$AutoFunc@$MH00@@YA?A?<auto>@@XZ",
        "<auto> __cdecl AutoFunc<1>(void)",
    );
    test(
        "??$AutoFunc@$00@@YA?A?<auto>@@XZ",
        "<auto> __cdecl AutoFunc<1>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$0A@@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<0>::AutoNTTPClass<0>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$MH0A@@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<0>::AutoNTTPClass<0>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$0A@$0A@$0GB@@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<0, 0, 97>::AutoNTTPClass<0, 0, 97>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$MH0A@$M_N0A@$MD0GB@@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<0, 0, 97>::AutoNTTPClass<0, 0, 97>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$M$$T0A@@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<0>::AutoNTTPClass<0>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$0A@@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<0>::AutoNTTPClass<0>(void)",
    );
    test("??0?$AutoNTTPClass@$MP8S@@EAAXXZ1?f@1@QEAAXXZ@@QEAA@XZ", "public: __cdecl AutoNTTPClass<&public: void __cdecl S::f(void)>::AutoNTTPClass<&public: void __cdecl S::f(void)>(void)");
    test("??0?$AutoNTTPClass@$1?f@S@@QEAAXXZ@@QEAA@XZ", "public: __cdecl AutoNTTPClass<&public: void __cdecl S::f(void)>::AutoNTTPClass<&public: void __cdecl S::f(void)>(void)");
    test("??0?$AutoNTTPClass@$MP8M@@EAAXXZH?f@1@QEAAXXZA@@@QEAA@XZ", "public: __cdecl AutoNTTPClass<{public: void __cdecl M::f(void), 0}>::AutoNTTPClass<{public: void __cdecl M::f(void), 0}>(void)");
    test("??0?$AutoNTTPClass@$H?f@M@@QEAAXXZA@@@QEAA@XZ", "public: __cdecl AutoNTTPClass<{public: void __cdecl M::f(void), 0}>::AutoNTTPClass<{public: void __cdecl M::f(void), 0}>(void)");
    test("??0?$AutoNTTPClass@$MP8V@@EAAXXZI?f@1@QEAAXXZA@A@@@QEAA@XZ", "public: __cdecl AutoNTTPClass<{public: void __cdecl V::f(void), 0, 0}>::AutoNTTPClass<{public: void __cdecl V::f(void), 0, 0}>(void)");
    test("??0?$AutoNTTPClass@$I?f@V@@QEAAXXZA@A@@@QEAA@XZ", "public: __cdecl AutoNTTPClass<{public: void __cdecl V::f(void), 0, 0}>::AutoNTTPClass<{public: void __cdecl V::f(void), 0, 0}>(void)");
    test(
        "??0?$AutoNTTPClass@$MPEQS@@H07@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<8>::AutoNTTPClass<8>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$07@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<8>::AutoNTTPClass<8>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$MPEQM@@H0M@@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<12>::AutoNTTPClass<12>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$0M@@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<12>::AutoNTTPClass<12>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$MPEQV@@HFBA@A@@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<{16, 0}>::AutoNTTPClass<{16, 0}>(void)",
    );
    test(
        "??0?$AutoNTTPClass@$FBA@A@@@QEAA@XZ",
        "public: __cdecl AutoNTTPClass<{16, 0}>::AutoNTTPClass<{16, 0}>(void)",
    );
}

#[test]
fn test_back_references() {
    test(
        "?f1@@YAXPBD0@Z",
        "void __cdecl f1(char const *, char const *)",
    );
    test("?f2@@YAXPBDPAD@Z", "void __cdecl f2(char const *, char *)");
    test(
        "?f3@@YAXHPBD0@Z",
        "void __cdecl f3(int, char const *, char const *)",
    );
    test(
        "?f4@@YAPBDPBD0@Z",
        "char const * __cdecl f4(char const *, char const *)",
    );
    test("?f5@@YAXPBDIDPBX0I@Z", "void __cdecl f5(char const *, unsigned int, char, void const *, char const *, unsigned int)");
    test("?f6@@YAX_N0@Z", "void __cdecl f6(bool, bool)");
    test(
        "?f7@@YAXHPAHH0_N1PA_N@Z",
        "void __cdecl f7(int, int *, int, int *, bool, bool, bool *)",
    );
    test("?g1@@YAXUS@@@Z", "void __cdecl g1(struct S)");
    test("?g2@@YAXUS@@0@Z", "void __cdecl g2(struct S, struct S)");
    test(
        "?g3@@YAXUS@@0PAU1@1@Z",
        "void __cdecl g3(struct S, struct S, struct S *, struct S *)",
    );
    test(
        "?g4@@YAXPBDPAUS@@01@Z",
        "void __cdecl g4(char const *, struct S *, char const *, struct S *)",
    );
    test("?mbb@S@@QAEX_N0@Z", "void __thiscall S::mbb(bool, bool)");
    test("?h1@@YAXPBD0P6AXXZ1@Z", "void __cdecl h1(char const *, char const *, void (__cdecl *)(void), void (__cdecl *)(void))");
    test(
        "?h2@@YAXP6AXPAX@Z0@Z",
        "void __cdecl h2(void (__cdecl *)(void *), void *)",
    );
    test("?h3@@YAP6APAHPAH0@ZP6APAH00@Z10@Z", "int * (__cdecl * __cdecl h3(int * (__cdecl *)(int *, int *), int * (__cdecl *)(int *, int *), int *))(int *, int *)");
    test("?foo@0@YAXXZ", "void __cdecl foo::foo(void)");
    test(
        "??$?HH@S@@QEAAAEAU0@H@Z",
        "struct S & __cdecl S::operator+<int>(int)",
    );
    test(
        "?foo_abbb@@YAXV?$A@V?$B@D@@V1@V1@@@@Z",
        "void __cdecl foo_abbb(class A<class B<char>, class B<char>, class B<char>>)",
    );
    test(
        "?foo_abb@@YAXV?$A@DV?$B@D@@V1@@@@Z",
        "void __cdecl foo_abb(class A<char, class B<char>, class B<char>>)",
    );
    test(
        "?foo_abc@@YAXV?$A@DV?$B@D@@V?$C@D@@@@@Z",
        "void __cdecl foo_abc(class A<char, class B<char>, class C<char>>)",
    );
    test(
        "?foo_bt@@YAX_NV?$B@$$A6A_N_N@Z@@@Z",
        "void __cdecl foo_bt(bool, class B<bool __cdecl(bool)>)",
    );
    test(
        "?foo_abbb@@YAXV?$A@V?$B@D@N@@V12@V12@@N@@@Z",
        "void __cdecl foo_abbb(class N::A<class N::B<char>, class N::B<char>, class N::B<char>>)",
    );
    test(
        "?foo_abb@@YAXV?$A@DV?$B@D@N@@V12@@N@@@Z",
        "void __cdecl foo_abb(class N::A<char, class N::B<char>, class N::B<char>>)",
    );
    test(
        "?foo_abc@@YAXV?$A@DV?$B@D@N@@V?$C@D@2@@N@@@Z",
        "void __cdecl foo_abc(class N::A<char, class N::B<char>, class N::C<char>>)",
    );
    test(
        "?abc_foo@@YA?AV?$A@DV?$B@D@N@@V?$C@D@2@@N@@XZ",
        "class N::A<char, class N::B<char>, class N::C<char>> __cdecl abc_foo(void)",
    );
    test(
        "?z_foo@@YA?AVZ@N@@V12@@Z",
        "class N::Z __cdecl z_foo(class N::Z)",
    );
    test(
        "?b_foo@@YA?AV?$B@D@N@@V12@@Z",
        "class N::B<char> __cdecl b_foo(class N::B<char>)",
    );
    test(
        "?d_foo@@YA?AV?$D@DD@N@@V12@@Z",
        "class N::D<char, char> __cdecl d_foo(class N::D<char, char>)",
    );
    test("?abc_foo_abc@@YA?AV?$A@DV?$B@D@N@@V?$C@D@2@@N@@V12@@Z", "class N::A<char, class N::B<char>, class N::C<char>> __cdecl abc_foo_abc(class N::A<char, class N::B<char>, class N::C<char>>)");
    test(
        "?foo5@@YAXV?$Y@V?$Y@V?$Y@V?$Y@VX@NA@@@NB@@@NA@@@NB@@@NA@@@Z",
        "void __cdecl foo5(class NA::Y<class NB::Y<class NA::Y<class NB::Y<class NA::X>>>>)",
    );
    test(
        "?foo11@@YAXV?$Y@VX@NA@@@NA@@V1NB@@@Z",
        "void __cdecl foo11(class NA::Y<class NA::X>, class NB::Y<class NA::X>)",
    );
    test(
        "?foo112@@YAXV?$Y@VX@NA@@@NA@@V?$Y@VX@NB@@@NB@@@Z",
        "void __cdecl foo112(class NA::Y<class NA::X>, class NB::Y<class NB::X>)",
    );
    test("?foo22@@YAXV?$Y@V?$Y@VX@NA@@@NB@@@NA@@V?$Y@V?$Y@VX@NA@@@NA@@@NB@@@Z", "void __cdecl foo22(class NA::Y<class NB::Y<class NA::X>>, class NB::Y<class NA::Y<class NA::X>>)");
    test(
        "?foo@L@PR13207@@QAEXV?$I@VA@PR13207@@@2@@Z",
        "void __thiscall PR13207::L::foo(class PR13207::I<class PR13207::A>)",
    );
    test(
        "?foo@PR13207@@YAXV?$I@VA@PR13207@@@1@@Z",
        "void __cdecl PR13207::foo(class PR13207::I<class PR13207::A>)",
    );
    test("?foo2@PR13207@@YAXV?$I@VA@PR13207@@@1@0@Z", "void __cdecl PR13207::foo2(class PR13207::I<class PR13207::A>, class PR13207::I<class PR13207::A>)");
    test(
        "?bar@PR13207@@YAXV?$J@VA@PR13207@@VB@2@@1@@Z",
        "void __cdecl PR13207::bar(class PR13207::J<class PR13207::A, class PR13207::B>)",
    );
    test("?spam@PR13207@@YAXV?$K@VA@PR13207@@VB@2@VC@2@@1@@Z", "void __cdecl PR13207::spam(class PR13207::K<class PR13207::A, class PR13207::B, class PR13207::C>)");
    test("?baz@PR13207@@YAXV?$K@DV?$F@D@PR13207@@V?$I@D@2@@1@@Z", "void __cdecl PR13207::baz(class PR13207::K<char, class PR13207::F<char>, class PR13207::I<char>>)");
    test("?qux@PR13207@@YAXV?$K@DV?$I@D@PR13207@@V12@@1@@Z", "void __cdecl PR13207::qux(class PR13207::K<char, class PR13207::I<char>, class PR13207::I<char>>)");
    test(
        "?foo@NA@PR13207@@YAXV?$Y@VX@NA@PR13207@@@12@@Z",
        "void __cdecl PR13207::NA::foo(class PR13207::NA::Y<class PR13207::NA::X>)",
    );
    test("?foofoo@NA@PR13207@@YAXV?$Y@V?$Y@VX@NA@PR13207@@@NA@PR13207@@@12@@Z", "void __cdecl PR13207::NA::foofoo(class PR13207::NA::Y<class PR13207::NA::Y<class PR13207::NA::X>>)");
    test(
        "?foo@NB@PR13207@@YAXV?$Y@VX@NA@PR13207@@@12@@Z",
        "void __cdecl PR13207::NB::foo(class PR13207::NB::Y<class PR13207::NA::X>)",
    );
    test(
        "?bar@NB@PR13207@@YAXV?$Y@VX@NB@PR13207@@@NA@2@@Z",
        "void __cdecl PR13207::NB::bar(class PR13207::NA::Y<class PR13207::NB::X>)",
    );
    test(
        "?spam@NB@PR13207@@YAXV?$Y@VX@NA@PR13207@@@NA@2@@Z",
        "void __cdecl PR13207::NB::spam(class PR13207::NA::Y<class PR13207::NA::X>)",
    );
    test("?foobar@NB@PR13207@@YAXV?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NA@2@V312@@Z", "void __cdecl PR13207::NB::foobar(class PR13207::NA::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>)");
    test("?foobarspam@NB@PR13207@@YAXV?$Y@VX@NB@PR13207@@@12@V?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NA@2@V412@@Z", "void __cdecl PR13207::NB::foobarspam(class PR13207::NB::Y<class PR13207::NB::X>, class PR13207::NA::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>)");
    test("?foobarbaz@NB@PR13207@@YAXV?$Y@VX@NB@PR13207@@@12@V?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NA@2@V412@2@Z", "void __cdecl PR13207::NB::foobarbaz(class PR13207::NB::Y<class PR13207::NB::X>, class PR13207::NA::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>)");
    test("?foobarbazqux@NB@PR13207@@YAXV?$Y@VX@NB@PR13207@@@12@V?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NA@2@V412@2V?$Y@V?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NB@PR13207@@@52@@Z", "void __cdecl PR13207::NB::foobarbazqux(class PR13207::NB::Y<class PR13207::NB::X>, class PR13207::NA::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NA::Y<class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>>)");
    test(
        "?foo@NC@PR13207@@YAXV?$Y@VX@NB@PR13207@@@12@@Z",
        "void __cdecl PR13207::NC::foo(class PR13207::NC::Y<class PR13207::NB::X>)",
    );
    test("?foobar@NC@PR13207@@YAXV?$Y@V?$Y@V?$Y@VX@NA@PR13207@@@NA@PR13207@@@NB@PR13207@@@12@@Z", "void __cdecl PR13207::NC::foobar(class PR13207::NC::Y<class PR13207::NB::Y<class PR13207::NA::Y<class PR13207::NA::X>>>)");
    test(
        "?fun_normal@fn_space@@YA?AURetVal@1@H@Z",
        "struct fn_space::RetVal __cdecl fn_space::fun_normal(int)",
    );
    test(
        "??$fun_tmpl@H@fn_space@@YA?AURetVal@0@ABH@Z",
        "struct fn_space::RetVal __cdecl fn_space::fun_tmpl<int>(int const &)",
    );
    test("??$fun_tmpl_recurse@H$1??$fun_tmpl_recurse@H$1?ident@fn_space@@YA?AURetVal@2@H@Z@fn_space@@YA?AURetVal@1@H@Z@fn_space@@YA?AURetVal@0@H@Z", "struct fn_space::RetVal __cdecl fn_space::fun_tmpl_recurse<int, &struct fn_space::RetVal __cdecl fn_space::fun_tmpl_recurse<int, &struct fn_space::RetVal __cdecl fn_space::ident(int)>(int)>(int)");
    test("??$fun_tmpl_recurse@H$1?ident@fn_space@@YA?AURetVal@2@H@Z@fn_space@@YA?AURetVal@0@H@Z", "struct fn_space::RetVal __cdecl fn_space::fun_tmpl_recurse<int, &struct fn_space::RetVal __cdecl fn_space::ident(int)>(int)");
    test("?AddEmitPasses@EmitAssemblyHelper@?A0x43583946@@AEAA_NAEAVPassManager@legacy@llvm@@W4BackendAction@clang@@AEAVraw_pwrite_stream@5@PEAV85@@Z", "bool __cdecl `anonymous namespace'::EmitAssemblyHelper::AddEmitPasses(class llvm::legacy::PassManager &, enum clang::BackendAction, class llvm::raw_pwrite_stream &, class llvm::raw_pwrite_stream *)");
    test("??$forward@P8?$DecoderStream@$01@media@@AEXXZ@std@@YA$$QAP8?$DecoderStream@$01@media@@AEXXZAAP812@AEXXZ@Z", "void (__thiscall media::DecoderStream<2>::*&& __cdecl std::forward<void (__thiscall media::DecoderStream<2>::*)(void)>(void (__thiscall media::DecoderStream<2>::*&)(void)))(void)");
}

#[test]
fn test_basic() {
    test("?x@@3HA", "int x");
    test("?x@@3PEAHEA", "int *x");
    test("?x@@3PEAPEAHEA", "int **x");
    test("?foo@@3Y123KA", "unsigned long foo[3][4]");
    test("?x@@3PEAY02HEA", "int (*x)[3]");
    test("?x@@3PEAY124HEA", "int (*x)[3][5]");
    test("?x@@3PEAY02$$CBHEA", "int const (*x)[3]");
    test("?x@@3PEAEEA", "unsigned char *x");
    test("?y@@3PEAGEA", "unsigned short *y");
    test("?z@@3PEAKEA", "unsigned long *z");
    test("?x@@3PEAY1NKM@5HEA", "int (*x)[3500][6]");
    test("?x@@YAXMH@Z", "void __cdecl x(float, int)");
    test("?x@@YAXMHZZ", "void __cdecl x(float, int, ...)");
    test("?x@@YAXZZ", "void __cdecl x(...)");
    test("?x@@3P6AHMNH@ZEA", "int (__cdecl *x)(float, double, int)");
    test(
        "?x@@3P6AHP6AHM@ZN@ZEA",
        "int (__cdecl *x)(int (__cdecl *)(float), double)",
    );
    test(
        "?x@@3P6AHP6AHM@Z0@ZEA",
        "int (__cdecl *x)(int (__cdecl *)(float), int (__cdecl *)(float))",
    );
    test("?x@ns@@3HA", "int ns::x");
    test("?x@@3PEAHEA", "int *x");
    test("?x@@3PEBHEB", "int const *x");
    test("?x@@3QEAHEA", "int *const x");
    test("?x@@3QEBHEB", "int const *const x");
    test("?x@@3AEBHEB", "int const &x");
    test("?x@@3PEAUty@@EA", "struct ty *x");
    test("?x@@3PEATty@@EA", "union ty *x");
    test("?x@@3PEAVty@@EA", "class ty *x");
    test("?x@@3PEAW4ty@@EA", "enum ty *x");
    test("?x@@3PEAV?$tmpl@H@@EA", "class tmpl<int> *x");
    test("?x@@3PEAU?$tmpl@H@@EA", "struct tmpl<int> *x");
    test("?x@@3PEAT?$tmpl@H@@EA", "union tmpl<int> *x");
    test("?instance@@3Vklass@@A", "class klass instance");
    test(
        "?instance$initializer$@@3P6AXXZEA",
        "void (__cdecl *instance$initializer$)(void)",
    );
    test("??0klass@@QEAA@XZ", "__cdecl klass::klass(void)");
    test("??1klass@@QEAA@XZ", "__cdecl klass::~klass(void)");
    test(
        "?x@@YAHPEAVklass@@AEAV1@@Z",
        "int __cdecl x(class klass *, class klass &)",
    );
    test(
        "?x@ns@@3PEAV?$klass@HH@1@EA",
        "class ns::klass<int, int> *ns::x",
    );
    test(
        "?fn@?$klass@H@ns@@QEBAIXZ",
        "unsigned int __cdecl ns::klass<int>::fn(void) const",
    );
    test(
        "??4klass@@QEAAAEBV0@AEBV0@@Z",
        "class klass const & __cdecl klass::operator=(class klass const &)",
    );
    test("??7klass@@QEAA_NXZ", "bool __cdecl klass::operator!(void)");
    test(
        "??8klass@@QEAA_NAEBV0@@Z",
        "bool __cdecl klass::operator==(class klass const &)",
    );
    test(
        "??9klass@@QEAA_NAEBV0@@Z",
        "bool __cdecl klass::operator!=(class klass const &)",
    );
    test(
        "??Aklass@@QEAAH_K@Z",
        "int __cdecl klass::operator[](unsigned __int64)",
    );
    test("??Cklass@@QEAAHXZ", "int __cdecl klass::operator->(void)");
    test("??Dklass@@QEAAHXZ", "int __cdecl klass::operator*(void)");
    test("??Eklass@@QEAAHXZ", "int __cdecl klass::operator++(void)");
    test("??Eklass@@QEAAHH@Z", "int __cdecl klass::operator++(int)");
    test("??Fklass@@QEAAHXZ", "int __cdecl klass::operator--(void)");
    test("??Fklass@@QEAAHH@Z", "int __cdecl klass::operator--(int)");
    test("??Hklass@@QEAAHH@Z", "int __cdecl klass::operator+(int)");
    test("??Gklass@@QEAAHH@Z", "int __cdecl klass::operator-(int)");
    test("??Iklass@@QEAAHH@Z", "int __cdecl klass::operator&(int)");
    test("??Jklass@@QEAAHH@Z", "int __cdecl klass::operator->*(int)");
    test("??Kklass@@QEAAHH@Z", "int __cdecl klass::operator/(int)");
    test("??Mklass@@QEAAHH@Z", "int __cdecl klass::operator<(int)");
    test("??Nklass@@QEAAHH@Z", "int __cdecl klass::operator<=(int)");
    test("??Oklass@@QEAAHH@Z", "int __cdecl klass::operator>(int)");
    test("??Pklass@@QEAAHH@Z", "int __cdecl klass::operator>=(int)");
    test("??Qklass@@QEAAHH@Z", "int __cdecl klass::operator,(int)");
    test("??Rklass@@QEAAHH@Z", "int __cdecl klass::operator()(int)");
    test("??Sklass@@QEAAHXZ", "int __cdecl klass::operator~(void)");
    test("??Tklass@@QEAAHH@Z", "int __cdecl klass::operator^(int)");
    test("??Uklass@@QEAAHH@Z", "int __cdecl klass::operator|(int)");
    test("??Vklass@@QEAAHH@Z", "int __cdecl klass::operator&&(int)");
    test("??Wklass@@QEAAHH@Z", "int __cdecl klass::operator||(int)");
    test("??Xklass@@QEAAHH@Z", "int __cdecl klass::operator*=(int)");
    test("??Yklass@@QEAAHH@Z", "int __cdecl klass::operator+=(int)");
    test("??Zklass@@QEAAHH@Z", "int __cdecl klass::operator-=(int)");
    test("??_0klass@@QEAAHH@Z", "int __cdecl klass::operator/=(int)");
    test("??_1klass@@QEAAHH@Z", "int __cdecl klass::operator%=(int)");
    test("??_2klass@@QEAAHH@Z", "int __cdecl klass::operator>>=(int)");
    test("??_3klass@@QEAAHH@Z", "int __cdecl klass::operator<<=(int)");
    test("??_6klass@@QEAAHH@Z", "int __cdecl klass::operator^=(int)");
    test(
        "??6@YAAEBVklass@@AEBV0@H@Z",
        "class klass const & __cdecl operator<<(class klass const &, int)",
    );
    test(
        "??5@YAAEBVklass@@AEBV0@_K@Z",
        "class klass const & __cdecl operator>>(class klass const &, unsigned __int64)",
    );
    test(
        "??2@YAPEAX_KAEAVklass@@@Z",
        "void * __cdecl operator new(unsigned __int64, class klass &)",
    );
    test(
        "??_U@YAPEAX_KAEAVklass@@@Z",
        "void * __cdecl operator new[](unsigned __int64, class klass &)",
    );
    test(
        "??3@YAXPEAXAEAVklass@@@Z",
        "void __cdecl operator delete(void *, class klass &)",
    );
    test(
        "??_V@YAXPEAXAEAVklass@@@Z",
        "void __cdecl operator delete[](void *, class klass &)",
    );
    test(
        "?A@?A0x43583946@@3VB@@B",
        "class B const `anonymous namespace'::A",
    );
}

#[test]
fn test_conversion_operators() {
    test(
        "??$?BH@TemplateOps@@QAEHXZ",
        "int __thiscall TemplateOps::operator<int> int(void)",
    );
    test("??BOps@@QAEHXZ", "int __thiscall Ops::operator int(void)");
    test(
        "??BConstOps@@QAE?BHXZ",
        "int const __thiscall ConstOps::operator int const(void)",
    );
    test(
        "??BVolatileOps@@QAE?CHXZ",
        "int volatile __thiscall VolatileOps::operator int volatile(void)",
    );
    test(
        "??BConstVolatileOps@@QAE?DHXZ",
        "int const volatile __thiscall ConstVolatileOps::operator int const volatile(void)",
    );
    test(
        "??$?BN@TemplateOps@@QAENXZ",
        "double __thiscall TemplateOps::operator<double> double(void)",
    );
    test(
        "??BOps@@QAENXZ",
        "double __thiscall Ops::operator double(void)",
    );
    test(
        "??BConstOps@@QAE?BNXZ",
        "double const __thiscall ConstOps::operator double const(void)",
    );
    test(
        "??BVolatileOps@@QAE?CNXZ",
        "double volatile __thiscall VolatileOps::operator double volatile(void)",
    );
    test(
        "??BConstVolatileOps@@QAE?DNXZ",
        "double const volatile __thiscall ConstVolatileOps::operator double const volatile(void)",
    );
    test(
        "??BCompoundTypeOps@@QAEPAHXZ",
        "nt * __thiscall CompoundTypeOps::operator int *(void)",
    );
    test(
        "??BCompoundTypeOps@@QAEPBHXZ",
        "int const * __thiscall CompoundTypeOps::operator int const *(void)",
    );
    test(
        "??BCompoundTypeOps@@QAE$$QAHXZ",
        "int && __thiscall CompoundTypeOps::operator int &&(void)",
    );
    test(
        "??BCompoundTypeOps@@QAE?AU?$Foo@H@@XZ",
        "struct Foo<int> __thiscall CompoundTypeOps::operator struct Foo<int>(void)",
    );
    test("??$?BH@CompoundTypeOps@@QAE?AU?$Bar@U?$Foo@H@@@@XZ", "struct Bar<struct Foo<int>> __thiscall CompoundTypeOps::operator<int> struct Bar<struct Foo<int>>(void)");
    test(
        "??$?BPAH@TemplateOps@@QAEPAHXZ",
        "int * __thiscall TemplateOps::operator<int *> int *(void)",
    );
}

#[test]
fn test_cxx11() {
    test(
        "?a@FTypeWithQuals@@3U?$S@$$A8@@BAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) const> FTypeWithQuals::a",
    );
    test(
        "?b@FTypeWithQuals@@3U?$S@$$A8@@CAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) volatile> FTypeWithQuals::b",
    );
    test(
        "?c@FTypeWithQuals@@3U?$S@$$A8@@IAAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) __restrict> FTypeWithQuals::c",
    );
    test(
        "?d@FTypeWithQuals@@3U?$S@$$A8@@GBAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) const &> FTypeWithQuals::d",
    );
    test(
        "?e@FTypeWithQuals@@3U?$S@$$A8@@GCAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) volatile &> FTypeWithQuals::e",
    );
    test(
        "?f@FTypeWithQuals@@3U?$S@$$A8@@IGAAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) __restrict &> FTypeWithQuals::f",
    );
    test(
        "?g@FTypeWithQuals@@3U?$S@$$A8@@HBAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) const &&> FTypeWithQuals::g",
    );
    test(
        "?h@FTypeWithQuals@@3U?$S@$$A8@@HCAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) volatile &&> FTypeWithQuals::h",
    );
    test(
        "?i@FTypeWithQuals@@3U?$S@$$A8@@IHAAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) __restrict &&> FTypeWithQuals::i",
    );
    test(
        "?j@FTypeWithQuals@@3U?$S@$$A6AHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::j",
    );
    test(
        "?k@FTypeWithQuals@@3U?$S@$$A8@@GAAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) &> FTypeWithQuals::k",
    );
    test(
        "?l@FTypeWithQuals@@3U?$S@$$A8@@HAAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void) &&> FTypeWithQuals::l",
    );
    test("?Char16Var@@3_SA", "char16_t Char16Var");
    test("?Char32Var@@3_UA", "char32_t Char32Var");
    test("?LRef@@YAXAAH@Z", "void __cdecl LRef(int &)");
    test("?RRef@@YAH$$QAH@Z", "int __cdecl RRef(int &&)");
    test("?Null@@YAX$$T@Z", "void __cdecl Null(std::nullptr_t)");
    test("?fun@PR18022@@YA?AU<unnamed-type-a>@1@U21@0@Z", "struct PR18022::<unnamed-type-a> __cdecl PR18022::fun(struct PR18022::<unnamed-type-a>, struct PR18022::<unnamed-type-a>)");
    test("?lambda@?1??define_lambda@@YAHXZ@4V<lambda_1>@?0??1@YAHXZ@A", "class `int __cdecl define_lambda(void)'::`1'::<lambda_1> `int __cdecl define_lambda(void)'::`2'::lambda");
    test(
        "??R<lambda_1>@?0??define_lambda@@YAHXZ@QBE@XZ",
        "__thiscall `int __cdecl define_lambda(void)'::`1'::<lambda_1>::operator()(void) const",
    );
    test(
        "?local@?2???R<lambda_1>@?0??define_lambda@@YAHXZ@QBE@XZ@4HA",
        "__thiscall `int __cdecl define_lambda(void)'::`1'::<lambda_1>::operator()(void) const",
    );
    test("??$use_lambda_arg@V<lambda_1>@?0??call_with_lambda_arg1@@YAXXZ@@@YAXV<lambda_1>@?0??call_with_lambda_arg1@@YAXXZ@@Z", "void __cdecl use_lambda_arg<class `void __cdecl call_with_lambda_arg1(void)'::`1'::<lambda_1>>(class `void __cdecl call_with_lambda_arg1(void)'::`1'::<lambda_1>)");
    test(
        "?foo@A@PR19361@@QIGAEXXZ",
        "void __thiscall PR19361::A::foo(void) __restrict &",
    );
    test(
        "?foo@A@PR19361@@QIHAEXXZ",
        "void __thiscall PR19361::A::foo(void) __restrict &&",
    );
    test(
        "??__K_deg@@YAHO@Z",
        "int __cdecl operator \"\"_deg(long double)",
    );
    test(
        "??$templ_fun_with_pack@$S@@YAXXZ",
        "void __cdecl templ_fun_with_pack<>(void)",
    );
    test(
        "??$func@H$$ZH@@YAHAEBU?$Foo@H@@0@Z",
        "int __cdecl func<int, int>(struct Foo<int> const &, struct Foo<int> const &)",
    );
    test(
        "??$templ_fun_with_ty_pack@$$$V@@YAXXZ",
        "void __cdecl templ_fun_with_ty_pack<>(void)",
    );
    test(
        "??$templ_fun_with_ty_pack@$$V@@YAXXZ",
        "void __cdecl templ_fun_with_ty_pack<>(void)",
    );
    test(
        "??$f@$$YAliasA@PR20047@@@PR20047@@YAXXZ",
        "void __cdecl PR20047::f<PR20047::AliasA>(void)",
    );
    test(
        "?f@UnnamedType@@YAXAAU<unnamed-type-TD>@A@1@@Z",
        "void __cdecl UnnamedType::f(struct UnnamedType::A::<unnamed-type-TD> &)",
    );
    test(
        "?f@UnnamedType@@YAXPAW4<unnamed-type-e>@?$B@H@1@@Z",
        "void __cdecl UnnamedType::f(enum UnnamedType::B<int>::<unnamed-type-e> *)",
    );
    test("??$f@W4<unnamed-type-E>@?1??g@PR24651@@YAXXZ@@PR24651@@YAXW4<unnamed-type-E>@?1??g@0@YAXXZ@@Z", "void __cdecl PR24651::f<enum `void __cdecl PR24651::g(void)'::`2'::<unnamed-type-E>>(enum `void __cdecl PR24651::g(void)'::`2'::<unnamed-type-E>)");
    test("??$f@T<unnamed-type-$S1>@PR18204@@@PR18204@@YAHPAT<unnamed-type-$S1>@0@@Z", "int __cdecl PR18204::f<union PR18204::<unnamed-type-$S1>>(union PR18204::<unnamed-type-$S1> *)");
    test(
        "??R<lambda_0>@?0??PR26105@@YAHXZ@QBE@H@Z",
        "public: __thiscall `int __cdecl PR26105(void)'::`1'::<lambda_0>::operator()(int) const",
    );
    test("??R<lambda_1>@?0???R<lambda_0>@?0??PR26105@@YAHXZ@QBE@H@Z@QBE@H@Z", "public: __thiscall `public: __thiscall `int __cdecl PR26105(void)'::`1'::<lambda_0>::operator()(int) const'::`1'::<lambda_1>::operator()(int) const");
    test(
        "?unaligned_foo1@@YAPFAHXZ",
        "int __unaligned * __cdecl unaligned_foo1(void)",
    );
    test(
        "?unaligned_foo2@@YAPFAPFAHXZ",
        "int __unaligned *__unaligned * __cdecl unaligned_foo2(void)",
    );
    test("?unaligned_foo3@@YAHXZ", "int __cdecl unaligned_foo3(void)");
    test(
        "?unaligned_foo4@@YAXPFAH@Z",
        "void __cdecl unaligned_foo4(int __unaligned *)",
    );
    test(
        "?unaligned_foo5@@YAXPIFAH@Z",
        "void __cdecl unaligned_foo5(int __unaligned *__restrict)",
    );
    test(
        "??$unaligned_foo6@PAH@@YAPAHPAH@Z",
        "int * __cdecl unaligned_foo6<int *>(int *)",
    );
    test(
        "??$unaligned_foo6@PFAH@@YAPFAHPFAH@Z",
        "int __unaligned * __cdecl unaligned_foo6<int __unaligned *>(int __unaligned *)",
    );
    test(
        "?unaligned_foo8@unaligned_foo8_S@@QFCEXXZ",
        "void __thiscall unaligned_foo8_S::unaligned_foo8(void) volatile __unaligned",
    );
    test(
        "??R<lambda_1>@x@A@PR31197@@QBE@XZ",
        "__thiscall PR31197::A::x::<lambda_1>::operator()(void) const",
    );
    test(
        "?white@?1???R<lambda_1>@x@A@PR31197@@QBE@XZ@4HA",
        "int `public: __thiscall PR31197::A::x::<lambda_1>::operator()(void) const'::`2'::white",
    );
    test(
        "?f@@YAXW4<unnamed-enum-enumerator>@@@Z",
        "void __cdecl f(enum <unnamed-enum-enumerator>)",
    );
}

#[test]
fn test_cxx14() {
    test("??$x@X@@3HA", "int x<void>");
    test(
        "?FunctionWithLocalType@@YA?A?<auto>@@XZ",
        "<auto> __cdecl FunctionWithLocalType(void)",
    );
    test("?ValueFromFunctionWithLocalType@@3ULocalType@?1??FunctionWithLocalType@@YA?A?<auto>@@XZ@A", "struct `<auto> __cdecl FunctionWithLocalType(void)'::`2'::LocalType ValueFromFunctionWithLocalType");
    test(
        "??R<lambda_0>@@QBE?A?<auto>@@XZ",
        "<auto> __thiscall <lambda_0>::operator()(void) const",
    );
    test("?ValueFromLambdaWithLocalType@@3ULocalType@?1???R<lambda_0>@@QBE?A?<auto>@@XZ@A", "struct `public: <auto> __thiscall <lambda_0>::operator()(void) const'::`2'::LocalType ValueFromLambdaWithLocalType");
    test("?ValueFromTemplateFuncionWithLocalLambda@@3ULocalType@?2???R<lambda_1>@?0???$TemplateFuncionWithLocalLambda@H@@YA?A?<auto>@@H@Z@QBE?A?3@XZ@A", "struct `public: <auto> __thiscall `<auto> __cdecl TemplateFuncionWithLocalLambda<int>(int)'::`1'::<lambda_1>::operator()(void) const'::`3'::LocalType ValueFromTemplateFuncionWithLocalLambda");
    test(
        "??$TemplateFuncionWithLocalLambda@H@@YA?A?<auto>@@H@Z",
        "<auto> __cdecl TemplateFuncionWithLocalLambda<int>(int)",
    );
    test("??R<lambda_1>@?0???$TemplateFuncionWithLocalLambda@H@@YA?A?<auto>@@H@Z@QBE?A?1@XZ", "<auto> __thiscall `<auto> __cdecl TemplateFuncionWithLocalLambda<int>(int)'::`1'::<lambda_1>::operator()(void) const");
    test("??$WithPMD@$GA@A@?0@@3HA", "int WithPMD<{0, 0, -1}>");
    test(
        "?Zoo@@3U?$Foo@$1??$x@H@@3HA$1?1@3HA@@A",
        "struct Foo<&int x<int>, &int x<int>> Zoo",
    );
    test(
        "??$unaligned_x@PFAH@@3PFAHA",
        "int __unaligned *unaligned_x<int __unaligned *>",
    );
}

#[test]
fn test_cxx17_noexcept() {
    test("?nochange@@YAXXZ", "void __cdecl nochange(void)");
    test("?a@@YAXP6AHXZ@Z", "void __cdecl a(int (__cdecl *)(void))");
    test(
        "?a@@YAXP6AHX_E@Z",
        "void __cdecl a(int (__cdecl *)(void) noexcept)",
    );
    test("?b@@YAXP6AHXZ@Z", "void __cdecl b(int (__cdecl *)(void))");
    test("?c@@YAXP6AHXZ@Z", "void __cdecl c(int (__cdecl *)(void))");
    test(
        "?c@@YAXP6AHX_E@Z",
        "void __cdecl c(int (__cdecl *)(void) noexcept)",
    );
    test(
        "?ee@?$e@$$A6AXXZ@@EEAAXXZ",
        "private: virtual void __cdecl e<void __cdecl(void)>::ee(void)",
    );
    test(
        "?ee@?$e@$$A6AXX_E@@EEAAXXZ",
        "private: virtual void __cdecl e<void __cdecl(void) noexcept>::ee(void)",
    );
}

#[test]
fn test_cxx20() {
    test(
        "??__LA@@QEAA?AUno_suspend@@XZ",
        "struct no_suspend __cdecl A::operator co_await(void)",
    );
    test(
        "??__MS@@QEAA?AVstrong_ordering@std@@AEBU0@@Z'",
        "class std::strong_ordering __cdecl S::operator<=>(struct S const &)",
    );
    test("?f@@YAX_Q@Z", "void __cdecl f(char8_t)");
}

#[test]
fn test_mangle() {
    test("?a@@3HA", "int a");
    test("?b@N@@3HA", "int N::b");
    test(
        "?anonymous@?A@N@@3HA",
        "int N::`anonymous namespace'::anonymous",
    );
    test(
        "?$RT1@NeedsReferenceTemporary@@3ABHB",
        "int const &NeedsReferenceTemporary::$RT1",
    );
    test(
        "?$RT1@NeedsReferenceTemporary@@3AEBHEB",
        "int const &NeedsReferenceTemporary::$RT1",
    );
    test("?_c@@YAHXZ", "int __cdecl _c(void)");
    test("?d@foo@@0FB", "static short const foo::d");
    test("?e@foo@@1JC", "static long volatile foo::e");
    test("?f@foo@@2DD", "static char const volatile foo::f");
    test("??0foo@@QAE@XZ", "__thiscall foo::foo(void)");
    test("??0foo@@QEAA@XZ", "__cdecl foo::foo(void)");
    test("??1foo@@QAE@XZ", "__thiscall foo::~foo(void)");
    test("??1foo@@QEAA@XZ", "__cdecl foo::~foo(void)");
    test("??0foo@@QAE@H@Z", "__thiscall foo::foo(int)");
    test("??0foo@@QEAA@H@Z", "__cdecl foo::foo(int)");
    test("??0foo@@QAE@PAD@Z", "__thiscall foo::foo(char *)");
    test("??0foo@@QEAA@PEAD@Z", "__cdecl foo::foo(char *)");
    test("?bar@@YA?AVfoo@@XZ", "class foo __cdecl bar(void)");
    test("?bar@@YA?AVfoo@@XZ", "class foo __cdecl bar(void)");
    test("??Hfoo@@QAEHH@Z", "int __thiscall foo::operator+(int)");
    test("??Hfoo@@QEAAHH@Z", "int __cdecl foo::operator+(int)");
    test(
        "??$?HH@S@@QEAAAEANH@Z",
        "double & __cdecl S::operator+<int>(int)",
    );
    test(
        "?static_method@foo@@SAPAV1@XZ",
        "static class foo * __cdecl foo::static_method(void)",
    );
    test(
        "?static_method@foo@@SAPEAV1@XZ",
        "static class foo * __cdecl foo::static_method(void)",
    );
    test("?g@bar@@2HA", "static int bar::g");
    test("?h1@@3QAHA", "int *const h1");
    test("?h2@@3QBHB", "int const *const h2");
    test("?h3@@3QIAHIA", "int *const __restrict h3");
    test("?h3@@3QEIAHEIA", "int *const __restrict h3");
    test("?i@@3PAY0BE@HA", "int (*i)[20]");
    test(
        "?FunArr@@3PAY0BE@P6AHHH@ZA",
        "int (__cdecl *(*FunArr)[20])(int, int)",
    );
    test(
        "?j@@3P6GHCE@ZA",
        "int (__stdcall *j)(signed char, unsigned char)",
    );
    test(
        "?funptr@@YAP6AHXZXZ",
        "int (__cdecl * __cdecl funptr(void))(void)",
    );
    test("?m@@3PRfoo@@DR1@", "char const foo::*m");
    test("?m@@3PERfoo@@DER1@", "char const foo::*m");
    test("?k@@3PTfoo@@DT1@", "char const volatile foo::*k");
    test("?k@@3PETfoo@@DET1@", "char const volatile foo::*k");
    test("?l@@3P8foo@@AEHH@ZQ1@", "int (__thiscall foo::*l)(int)");
    test("?g_cInt@@3HB", "int const g_cInt");
    test("?g_vInt@@3HC", "int volatile g_vInt");
    test("?g_cvInt@@3HD", "int const volatile g_cvInt");
    test(
        "?beta@@YI_N_J_W@Z",
        "bool __fastcall beta(__int64, wchar_t)",
    );
    test("?beta@@YA_N_J_W@Z", "bool __cdecl beta(__int64, wchar_t)");
    test("?alpha@@YGXMN@Z", "void __stdcall alpha(float, double)");
    test("?alpha@@YAXMN@Z", "void __cdecl alpha(float, double)");
    test(
        "?gamma@@YAXVfoo@@Ubar@@Tbaz@@W4quux@@@Z",
        "void __cdecl gamma(class foo, struct bar, union baz, enum quux)",
    );
    test(
        "?gamma@@YAXVfoo@@Ubar@@Tbaz@@W4quux@@@Z",
        "void __cdecl gamma(class foo, struct bar, union baz, enum quux)",
    );
    test(
        "?delta@@YAXQAHABJ@Z",
        "void __cdecl delta(int *const, long const &)",
    );
    test(
        "?delta@@YAXQEAHAEBJ@Z",
        "void __cdecl delta(int *const, long const &)",
    );
    test(
        "?epsilon@@YAXQAY19BE@H@Z",
        "void __cdecl epsilon(int (*const)[10][20])",
    );
    test(
        "?epsilon@@YAXQEAY19BE@H@Z",
        "void __cdecl epsilon(int (*const)[10][20])",
    );
    test(
        "?zeta@@YAXP6AHHH@Z@Z",
        "void __cdecl zeta(int (__cdecl *)(int, int))",
    );
    test(
        "?zeta@@YAXP6AHHH@Z@Z",
        "void __cdecl zeta(int (__cdecl *)(int, int))",
    );
    test("??2@YAPAXI@Z", "void * __cdecl operator new(unsigned int)");
    test("??3@YAXPAX@Z", "void __cdecl operator delete(void *)");
    test(
        "??_U@YAPAXI@Z",
        "void * __cdecl operator new[](unsigned int)",
    );
    test("??_V@YAXPAX@Z", "void __cdecl operator delete[](void *)");
    test("?color1@@3PANA", "double *color1");
    test("?color2@@3QBNB", "double const *const color2");
    test("?color3@@3QAY02$$CBNA", "double const (*const color3)[3]");
    test("?color4@@3QAY02$$CBNA", "double const (*const color4)[3]");
    test(
        "?memptr1@@3RESB@@HES1@",
        "int volatile B::*volatile memptr1",
    );
    test("?memptr2@@3PESB@@HES1@", "int volatile B::*memptr2");
    test("?memptr3@@3REQB@@HEQ1@", "int B::*volatile memptr3");
    test(
        "?funmemptr1@@3RESB@@R6AHXZES1@",
        "int (__cdecl *volatile B::*volatile funmemptr1)(void)",
    );
    test(
        "?funmemptr2@@3PESB@@R6AHXZES1@",
        "int (__cdecl *volatile B::*funmemptr2)(void)",
    );
    test(
        "?funmemptr3@@3REQB@@P6AHXZEQ1@",
        "int (__cdecl *B::*volatile funmemptr3)(void)",
    );
    test(
        "?memptrtofun1@@3R8B@@EAAXXZEQ1@",
        "void (__cdecl B::*volatile memptrtofun1)(void)",
    );
    test(
        "?memptrtofun2@@3P8B@@EAAXXZEQ1@",
        "void (__cdecl B::*memptrtofun2)(void)",
    );
    test(
        "?memptrtofun3@@3P8B@@EAAXXZEQ1@",
        "void (__cdecl B::*memptrtofun3)(void)",
    );
    test(
        "?memptrtofun4@@3R8B@@EAAHXZEQ1@",
        "int (__cdecl B::*volatile memptrtofun4)(void)",
    );
    test(
        "?memptrtofun5@@3P8B@@EAA?CHXZEQ1@",
        "int volatile (__cdecl B::*memptrtofun5)(void)",
    );
    test(
        "?memptrtofun6@@3P8B@@EAA?BHXZEQ1@",
        "int const (__cdecl B::*memptrtofun6)(void)",
    );
    test(
        "?memptrtofun7@@3R8B@@EAAP6AHXZXZEQ1@",
        "int (__cdecl * (__cdecl B::*volatile memptrtofun7)(void))(void)",
    );
    test(
        "?memptrtofun8@@3P8B@@EAAR6AHXZXZEQ1@",
        "int (__cdecl *volatile (__cdecl B::*memptrtofun8)(void))(void)",
    );
    test(
        "?memptrtofun9@@3P8B@@EAAQ6AHXZXZEQ1@",
        "int (__cdecl *const (__cdecl B::*memptrtofun9)(void))(void)",
    );
    test("?fooE@@YA?AW4E@@XZ", "enum E __cdecl fooE(void)");
    test("?fooE@@YA?AW4E@@XZ", "enum E __cdecl fooE(void)");
    test("?fooX@@YA?AVX@@XZ", "class X __cdecl fooX(void)");
    test("?fooX@@YA?AVX@@XZ", "class X __cdecl fooX(void)");
    test("?s0@PR13182@@3PADA", "char *PR13182::s0");
    test("?s1@PR13182@@3PADA", "char *PR13182::s1");
    test("?s2@PR13182@@3QBDB", "char const *const PR13182::s2");
    test("?s3@PR13182@@3QBDB", "char const *const PR13182::s3");
    test("?s4@PR13182@@3RCDC", "char volatile *volatile PR13182::s4");
    test(
        "?s5@PR13182@@3SDDD",
        "char const volatile *const volatile PR13182::s5",
    );
    test("?s6@PR13182@@3PBQBDB", "char const *const *PR13182::s6");
    test(
        "?local@?1??extern_c_func@@9@4HA",
        "int `extern \"C\" extern_c_func'::`2'::local",
    );
    test(
        "?local@?1??extern_c_func@@9@4HA",
        "int `extern \"C\" extern_c_func'::`2'::local",
    );
    test(
        "?v@?1??f@@YAHXZ@4U<unnamed-type-v>@?1??1@YAHXZ@A",
        "struct `int __cdecl f(void)'::`2'::<unnamed-type-v> `int __cdecl f(void)'::`2'::v",
    );
    test("?v@?1???$f@H@@YAHXZ@4U<unnamed-type-v>@?1???$f@H@@YAHXZ@A", "struct `int __cdecl f<int>(void)'::`2'::<unnamed-type-v> `int __cdecl f<int>(void)'::`2'::v");
    test(
        "??2OverloadedNewDelete@@SAPAXI@Z",
        "static void * __cdecl OverloadedNewDelete::operator new(unsigned int)",
    );
    test(
        "??_UOverloadedNewDelete@@SAPAXI@Z",
        "static void * __cdecl OverloadedNewDelete::operator new[](unsigned int)",
    );
    test(
        "??3OverloadedNewDelete@@SAXPAX@Z",
        "static void __cdecl OverloadedNewDelete::operator delete(void *)",
    );
    test(
        "??_VOverloadedNewDelete@@SAXPAX@Z",
        "static void __cdecl OverloadedNewDelete::operator delete[](void *)",
    );
    test(
        "??HOverloadedNewDelete@@QAEHH@Z",
        "int __thiscall OverloadedNewDelete::operator+(int)",
    );
    test(
        "??2OverloadedNewDelete@@SAPEAX_K@Z",
        "static void * __cdecl OverloadedNewDelete::operator new(unsigned __int64)",
    );
    test(
        "??_UOverloadedNewDelete@@SAPEAX_K@Z",
        "static void * __cdecl OverloadedNewDelete::operator new[](unsigned __int64)",
    );
    test(
        "??3OverloadedNewDelete@@SAXPEAX@Z",
        "static void __cdecl OverloadedNewDelete::operator delete(void *)",
    );
    test(
        "??_VOverloadedNewDelete@@SAXPEAX@Z",
        "static void __cdecl OverloadedNewDelete::operator delete[](void *)",
    );
    test(
        "??HOverloadedNewDelete@@QEAAHH@Z",
        "int __cdecl OverloadedNewDelete::operator+(int)",
    );
    test(
        "??2TypedefNewDelete@@SAPAXI@Z",
        "static void * __cdecl TypedefNewDelete::operator new(unsigned int)",
    );
    test(
        "??_UTypedefNewDelete@@SAPAXI@Z",
        "static void * __cdecl TypedefNewDelete::operator new[](unsigned int)",
    );
    test(
        "??3TypedefNewDelete@@SAXPAX@Z",
        "static void __cdecl TypedefNewDelete::operator delete(void *)",
    );
    test(
        "??_VTypedefNewDelete@@SAXPAX@Z",
        "static void __cdecl TypedefNewDelete::operator delete[](void *)",
    );
    test("?vector_func@@YQXXZ", "void __vectorcall vector_func(void)");
    test(
        "?swift_func@@YSXXZ",
        "void __attribute__((__swiftcall__)) swift_func(void)",
    );
    test(
        "?swift_async_func@@YWXXZ",
        "void __attribute__((__swiftasynccall__)) swift_async_func(void)",
    );
    test(
        "??$fn_tmpl@$1?extern_c_func@@YAXXZ@@YAXXZ",
        "void __cdecl fn_tmpl<&void __cdecl extern_c_func(void)>(void)",
    );
    test(
        "?overloaded_fn@@$$J0YAXXZ",
        "extern \"C\" void __cdecl overloaded_fn(void)",
    );
    test(
        "?f@UnnamedType@@YAXQAPAU<unnamed-type-T1>@S@1@@Z",
        "void __cdecl UnnamedType::f(struct UnnamedType::S::<unnamed-type-T1> **const)",
    );
    test(
        "?f@UnnamedType@@YAXUT2@S@1@@Z",
        "void __cdecl UnnamedType::f(struct UnnamedType::S::T2)",
    );
    test(
        "?f@UnnamedType@@YAXPAUT4@S@1@@Z",
        "void __cdecl UnnamedType::f(struct UnnamedType::S::T4 *)",
    );
    test(
        "?f@UnnamedType@@YAXUT4@S@1@@Z",
        "void __cdecl UnnamedType::f(struct UnnamedType::S::T4)",
    );
    test(
        "?f@UnnamedType@@YAXUT5@S@1@@Z",
        "void __cdecl UnnamedType::f(struct UnnamedType::S::T5)",
    );
    test(
        "?f@UnnamedType@@YAXUT2@S@1@@Z",
        "void __cdecl UnnamedType::f(struct UnnamedType::S::T2)",
    );
    test(
        "?f@UnnamedType@@YAXUT4@S@1@@Z",
        "void __cdecl UnnamedType::f(struct UnnamedType::S::T4)",
    );
    test(
        "?f@UnnamedType@@YAXUT5@S@1@@Z",
        "void __cdecl UnnamedType::f(struct UnnamedType::S::T5)",
    );
    test(
        "?f@Atomic@@YAXU?$_Atomic@H@__clang@@@Z",
        "void __cdecl Atomic::f(struct __clang::_Atomic<int>)",
    );
    test(
        "?f@Complex@@YAXU?$_Complex@H@__clang@@@Z",
        "void __cdecl Complex::f(struct __clang::_Complex<int>)",
    );
    test(
        "?f@Float16@@YAXU_Float16@__clang@@@Z",
        "void __cdecl Float16::f(struct __clang::_Float16)",
    );
    test("??0?$L@H@NS@@QEAA@XZ", "__cdecl NS::L<int>::L<int>(void)");
    test("??0Bar@Foo@@QEAA@XZ", "__cdecl Foo::Bar::Bar(void)");
    test(
        "??0?$L@V?$H@PAH@PR26029@@@PR26029@@QAE@XZ",
        "__thiscall PR26029::L<class PR26029::H<int *>>::L<class PR26029::H<int *>>(void)",
    );
    test("??$emplace_back@ABH@?$vector@HV?$allocator@H@std@@@std@@QAE?A?<decltype-auto>@@ABH@Z", "<decltype-auto> __thiscall std::vector<int, class std::allocator<int>>::emplace_back<int const &>(int const &)");
    test(
        "?pub_foo@S@@QAEXXZ",
        "public: void __thiscall S::pub_foo(void)",
    );
    test(
        "?pub_stat_foo@S@@SAXXZ",
        "public: static void __cdecl S::pub_stat_foo(void)",
    );
    test(
        "?pub_virt_foo@S@@UAEXXZ",
        "public: virtual void __thiscall S::pub_virt_foo(void)",
    );
    test(
        "?prot_foo@S@@IAEXXZ",
        "protected: void __thiscall S::prot_foo(void)",
    );
    test(
        "?prot_stat_foo@S@@KAXXZ",
        "protected: static void __cdecl S::prot_stat_foo(void)",
    );
    test(
        "?prot_virt_foo@S@@MAEXXZ",
        "protected: virtual void __thiscall S::prot_virt_foo(void)",
    );
    test(
        "?priv_foo@S@@AAEXXZ",
        "private: void __thiscall S::priv_foo(void)",
    );
    test(
        "?priv_stat_foo@S@@CAXXZ",
        "private: static void __cdecl S::priv_stat_foo(void)",
    );
    test(
        "?priv_virt_foo@S@@EAEXXZ",
        "private: virtual void __thiscall S::priv_virt_foo(void)",
    );
}

#[test]
fn test_md5() {
    test(
        "??@a6a285da2eea70dba6b578022be61d81@",
        "??@a6a285da2eea70dba6b578022be61d81@",
    );
    test(
        "??@a6a285da2eea70dba6b578022be61d81@asdf",
        "??@a6a285da2eea70dba6b578022be61d81@",
    );
    test(
        "??@a6a285da2eea70dba6b578022be61d81@??_R4@",
        "??@a6a285da2eea70dba6b578022be61d81@??_R4@",
    );
}

#[test]
fn test_nested_scopes() {
    test("?M@?@??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`0'::M");
    test("?M@?0??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`1'::M");
    test("?M@?1??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`2'::M");
    test("?M@?2??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`3'::M");
    test("?M@?3??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`4'::M");
    test("?M@?4??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`5'::M");
    test("?M@?5??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`6'::M");
    test("?M@?6??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`7'::M");
    test("?M@?7??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`8'::M");
    test("?M@?8??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`9'::M");
    test("?M@?9??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`10'::M");
    test("?M@?L@??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`11'::M");
    test("?M@?M@??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`12'::M");
    test("?M@?N@??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`13'::M");
    test("?M@?O@??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`14'::M");
    test("?M@?P@??L@@YAHXZ@4HA", "int `int __cdecl L(void)'::`15'::M");
    test(
        "?M@?BA@??L@@YAHXZ@4HA",
        "int `int __cdecl L(void)'::`16'::M",
    );
    test(
        "?M@?BB@??L@@YAHXZ@4HA",
        "int `int __cdecl L(void)'::`17'::M",
    );
    test(
        "?j@?1??L@@YAHXZ@4UJ@@A",
        "struct J `int __cdecl L(void)'::`2'::j",
    );
    test("?NN@0XX@@3HA", "int XX::NN::NN");
    test("?MM@0NN@XX@@3HA", "int XX::NN::MM::MM");
    test("?NN@MM@0XX@@3HA", "int XX::NN::MM::NN");
    test("?OO@0NN@01XX@@3HA", "int XX::NN::OO::NN::OO::OO");
    test("?NN@OO@010XX@@3HA", "int XX::NN::OO::NN::OO::NN");
    test("?M@?1??0@YAHXZ@4HA", "int `int __cdecl M(void)'::`2'::M");
    test(
        "?L@?2??M@0?2??0@YAHXZ@QEAAHXZ@4HA",
        "int `public: int __cdecl `int __cdecl L(void)'::`3'::L::M(void)'::`3'::L",
    );
    test(
        "?M@?2??0L@?2??1@YAHXZ@QEAAHXZ@4HA",
        "int `public: int __cdecl `int __cdecl L(void)'::`3'::L::M(void)'::`3'::M",
    );
    test(
        "?M@?1???$L@H@@YAHXZ@4HA",
        "int `int __cdecl L<int>(void)'::`2'::M",
    );
    test(
        "?SN@?$NS@H@NS@@QEAAHXZ",
        "int __cdecl NS::NS<int>::SN(void)",
    );
    test(
        "?NS@?1??SN@?$NS@H@0@QEAAHXZ@4HA",
        "int `public: int __cdecl NS::NS<int>::SN(void)'::`2'::NS",
    );
    test(
        "?SN@?1??0?$NS@H@NS@@QEAAHXZ@4HA",
        "int `public: int __cdecl NS::NS<int>::SN(void)'::`2'::SN",
    );
    test(
        "?NS@?1??SN@?$NS@H@10@QEAAHXZ@4HA",
        "int `public: int __cdecl NS::SN::NS<int>::SN(void)'::`2'::NS",
    );
    test(
        "?SN@?1??0?$NS@H@0NS@@QEAAHXZ@4HA",
        "int `public: int __cdecl NS::SN::NS<int>::SN(void)'::`2'::SN",
    );
    test("?X@?$C@H@C@0@2HB", "static int const X::C::C<int>::X");
    test("?X@?$C@H@C@1@2HB", "static int const C<int>::C::C<int>::X");
    test("?X@?$C@H@C@2@2HB", "static int const C::C::C<int>::X");
    test(
        "?C@?1??B@?$C@H@0101A@@QEAAHXZ@4U201013@A",
        "struct A::B::C::B::C::C<int> `public: int __cdecl A::B::C::B::C::C<int>::B(void)'::`2'::C",
    );
    test(
        "?B@?1??0?$C@H@C@020A@@QEAAHXZ@4HA",
        "int `public: int __cdecl A::B::C::B::C::C<int>::B(void)'::`2'::B",
    );
    test(
        "?A@?1??B@?$C@H@C@1310@QEAAHXZ@4HA",
        "int `public: int __cdecl A::B::C::B::C::C<int>::B(void)'::`2'::A",
    );
    test("?a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@@3HA", "int a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a");
}

#[test]
fn test_operators() {
    test("??0Base@@QEAA@XZ", "__cdecl Base::Base(void)");
    test("??1Base@@UEAA@XZ", "virtual __cdecl Base::~Base(void)");
    test(
        "??2@YAPEAX_K@Z",
        "void * __cdecl operator new(unsigned __int64)",
    );
    test(
        "??3@YAXPEAX_K@Z",
        "void __cdecl operator delete(void *, unsigned __int64)",
    );
    test("??4Base@@QEAAHH@Z", "int __cdecl Base::operator=(int)");
    test("??6Base@@QEAAHH@Z", "int __cdecl Base::operator<<(int)");
    test("??5Base@@QEAAHH@Z", "int __cdecl Base::operator>>(int)");
    test("??7Base@@QEAAHXZ", "int __cdecl Base::operator!(void)");
    test("??8Base@@QEAAHH@Z", "int __cdecl Base::operator==(int)");
    test("??9Base@@QEAAHH@Z", "int __cdecl Base::operator!=(int)");
    test("??ABase@@QEAAHH@Z", "int __cdecl Base::operator[](int)");
    test("??BBase@@QEAAHXZ", "__cdecl Base::operator int(void)");
    test("??CBase@@QEAAHXZ", "int __cdecl Base::operator->(void)");
    test("??DBase@@QEAAHXZ", "int __cdecl Base::operator*(void)");
    test("??EBase@@QEAAHXZ", "int __cdecl Base::operator++(void)");
    test("??EBase@@QEAAHH@Z", "int __cdecl Base::operator++(int)");
    test("??FBase@@QEAAHXZ", "int __cdecl Base::operator--(void)");
    test("??FBase@@QEAAHH@Z", "int __cdecl Base::operator--(int)");
    test("??GBase@@QEAAHH@Z", "int __cdecl Base::operator-(int)");
    test("??HBase@@QEAAHH@Z", "int __cdecl Base::operator+(int)");
    test("??IBase@@QEAAHH@Z", "int __cdecl Base::operator&(int)");
    test("??JBase@@QEAAHH@Z", "int __cdecl Base::operator->*(int)");
    test("??KBase@@QEAAHH@Z", "int __cdecl Base::operator/(int)");
    test("??LBase@@QEAAHH@Z", "int __cdecl Base::operator%(int)");
    test("??MBase@@QEAAHH@Z", "int __cdecl Base::operator<(int)");
    test("??NBase@@QEAAHH@Z", "int __cdecl Base::operator<=(int)");
    test("??OBase@@QEAAHH@Z", "int __cdecl Base::operator>(int)");
    test("??PBase@@QEAAHH@Z", "int __cdecl Base::operator>=(int)");
    test("??QBase@@QEAAHH@Z", "int __cdecl Base::operator,(int)");
    test("??RBase@@QEAAHXZ", "int __cdecl Base::operator()(void)");
    test("??SBase@@QEAAHXZ", "int __cdecl Base::operator~(void)");
    test("??TBase@@QEAAHH@Z", "int __cdecl Base::operator^(int)");
    test("??UBase@@QEAAHH@Z", "int __cdecl Base::operator|(int)");
    test("??VBase@@QEAAHH@Z", "int __cdecl Base::operator&&(int)");
    test("??WBase@@QEAAHH@Z", "int __cdecl Base::operator||(int)");
    test("??XBase@@QEAAHH@Z", "int __cdecl Base::operator*=(int)");
    test("??YBase@@QEAAHH@Z", "int __cdecl Base::operator+=(int)");
    test("??ZBase@@QEAAHH@Z", "int __cdecl Base::operator-=(int)");
    test("??_0Base@@QEAAHH@Z", "int __cdecl Base::operator/=(int)");
    test("??_1Base@@QEAAHH@Z", "int __cdecl Base::operator%=(int)");
    test("??_2Base@@QEAAHH@Z", "int __cdecl Base::operator>>=(int)");
    test("??_3Base@@QEAAHH@Z", "int __cdecl Base::operator<<=(int)");
    test("??_4Base@@QEAAHH@Z", "int __cdecl Base::operator&=(int)");
    test("??_5Base@@QEAAHH@Z", "int __cdecl Base::operator|=(int)");
    test("??_6Base@@QEAAHH@Z", "int __cdecl Base::operator^=(int)");
    test("??_7Base@@6B@", "const Base::`vftable'");
    test("??_7A@B@@6BC@D@@@", "const B::A::`vftable'{for `D::C'}");
    test("??_8Middle2@@7B@", "const Middle2::`vbtable'");
    test(
        "??_9Base@@$B7AA",
        "[thunk]: __cdecl Base::`vcall'{8, {flat}}",
    );
    test(
        "??_B?1??getS@@YAAAUS@@XZ@51",
        "`struct S & __cdecl getS(void)'::`2'::`local static guard'{2}",
    );
    test("??_C@_02PCEFGMJL@hi?$AA@", "\"hi\"");
    test(
        "??_DDiamond@@QEAAXXZ",
        "void __cdecl Diamond::`vbase dtor'(void)",
    );
    test(
        "??_EBase@@UEAAPEAXI@Z",
        "virtual void * __cdecl Base::`vector deleting dtor'(unsigned int)",
    );
    test("??_EBase@@G3AEPAXI@Z", "[thunk]: private: void * __thiscall Base::`vector deleting dtor'`adjustor{4}'(unsigned int)");
    test(
        "??_F?$SomeTemplate@H@@QAEXXZ",
        "void __thiscall SomeTemplate<int>::`default constructor closure'(void)",
    );
    test(
        "??_GBase@@UEAAPEAXI@Z",
        "virtual void * __cdecl Base::`scalar deleting dtor'(unsigned int)",
    );
    test("??_H@YAXPEAX_K1P6APEAX0@Z@Z", "void __cdecl `vector ctor iterator'(void *, unsigned __int64, unsigned __int64, void * (__cdecl *)(void *))");
    test("??_I@YAXPEAX_K1P6AX0@Z@Z", "void __cdecl `vector dtor iterator'(void *, unsigned __int64, unsigned __int64, void (__cdecl *)(void *))");
    test(
        "??_JBase@@UEAAPEAXI@Z",
        "virtual void * __cdecl Base::`vector vbase ctor iterator'(unsigned int)",
    );
    test(
        "??_KBase@@UEAAPEAXI@Z",
        "virtual void * __cdecl Base::`virtual displacement map'(unsigned int)",
    );
    test(
        "??_LBase@@UEAAPEAXI@Z",
        "virtual void * __cdecl Base::`eh vector ctor iterator'(unsigned int)",
    );
    test(
        "??_MBase@@UEAAPEAXI@Z",
        "virtual void * __cdecl Base::`eh vector dtor iterator'(unsigned int)",
    );
    test(
        "??_NBase@@UEAAPEAXI@Z",
        "virtual void * __cdecl Base::`eh vector vbase ctor iterator'(unsigned int)",
    );
    test(
        "??_O?$SomeTemplate@H@@QAEXXZ",
        "void __thiscall SomeTemplate<int>::`copy ctor closure'(void)",
    );
    test("??_SBase@@6B@", "const Base::`local vftable'");
    test(
        "??_TDerived@@QEAAXXZ",
        "void __cdecl Derived::`local vftable ctor closure'(void)",
    );
    test(
        "??_U@YAPEAX_KAEAVklass@@@Z",
        "void * __cdecl operator new[](unsigned __int64, class klass &)",
    );
    test(
        "??_V@YAXPEAXAEAVklass@@@Z",
        "void __cdecl operator delete[](void *, class klass &)",
    );
    test("??_R0?AUBase@@@8", "struct Base `RTTI Type Descriptor'");
    test(".?AUBase@@", "struct Base `RTTI Type Descriptor Name'");
    test(
        "??_R1A@?0A@EA@Base@@8",
        "Base::`RTTI Base Class Descriptor at (0, -1, 0, 64)'",
    );
    test("??_R2Base@@8", "Base::`RTTI Base Class Array'");
    test("??_R3Base@@8", "Base::`RTTI Class Hierarchy Descriptor'");
    test(
        "??_R4Base@@6B@",
        "const Base::`RTTI Complete Object Locator'",
    );
    test(
        "??__EFoo@@YAXXZ",
        "void __cdecl `dynamic initializer for 'Foo''(void)",
    );
    test(
        "??__E?i@C@@0HA@@YAXXZ",
        "void __cdecl `dynamic initializer for `private: static int C::i''(void)",
    );
    test(
        "??__FFoo@@YAXXZ",
        "void __cdecl `dynamic atexit destructor for 'Foo''(void)",
    );
    test("??__F_decisionToDFA@XPathLexer@@0V?$vector@VDFA@dfa@antlr4@@V?$allocator@VDFA@dfa@antlr4@@@std@@@std@@A@YAXXZ", "void __cdecl `dynamic atexit destructor for `private: static class std::vector<class antlr4::dfa::DFA, class std::allocator<class antlr4::dfa::DFA>> XPathLexer::_decisionToDFA''(void)");
    test(
        "??__J?1??f@@YAAAUS@@XZ@51",
        "`struct S & __cdecl f(void)'::`2'::`local static thread guard'{2}",
    );
    test(
        "??__K_deg@@YAHO@Z",
        "int __cdecl operator \"\"_deg(long double)",
    );
}

#[test]
fn test_options() {
    let test_options = |mangled_name: &str,
                        default: &str,
                        no_calling_conv: &str,
                        no_return: &str,
                        no_access: &str,
                        no_member_type: &str,
                        no_variable_type: &str,
                        no_all: &str| {
        do_test(mangled_name, default, false, Flags::default());
        do_test(
            mangled_name,
            no_calling_conv,
            false,
            Flags::NO_CALLING_CONVENTION,
        );
        do_test(mangled_name, no_return, false, Flags::NO_RETURN_TYPE);
        do_test(mangled_name, no_access, false, Flags::NO_ACCESS_SPECIFIER);
        do_test(mangled_name, no_member_type, false, Flags::NO_MEMBER_TYPE);
        do_test(
            mangled_name,
            no_variable_type,
            false,
            Flags::NO_VARIABLE_TYPE,
        );
        do_test(
            mangled_name,
            no_all,
            false,
            Flags::NO_CALLING_CONVENTION
                | Flags::NO_RETURN_TYPE
                | Flags::NO_ACCESS_SPECIFIER
                | Flags::NO_MEMBER_TYPE
                | Flags::NO_VARIABLE_TYPE,
        );
    };

    test_options(
        "?func@MyClass@@UEAAHHH@Z",
        "public: virtual int __cdecl MyClass::func(int, int)",
        "public: virtual int MyClass::func(int, int)",
        "public: virtual __cdecl MyClass::func(int, int)",
        "virtual int __cdecl MyClass::func(int, int)",
        "public: int __cdecl MyClass::func(int, int)",
        "public: virtual int __cdecl MyClass::func(int, int)",
        "MyClass::func(int, int)",
    );
    test_options(
        "?array2d@@3PAY09HA",
        "int (*array2d)[10]",
        "int (*array2d)[10]",
        "int (*array2d)[10]",
        "int (*array2d)[10]",
        "int (*array2d)[10]",
        "array2d",
        "array2d",
    );
    test_options(
        "?a@abc@@3PAY09HA",
        "int (*abc::a)[10]",
        "int (*abc::a)[10]",
        "int (*abc::a)[10]",
        "int (*abc::a)[10]",
        "int (*abc::a)[10]",
        "abc::a",
        "abc::a",
    );
    test_options(
        "?x@@3PEAEEA",
        "unsigned char *x",
        "unsigned char *x",
        "unsigned char *x",
        "unsigned char *x",
        "unsigned char *x",
        "x",
        "x",
    );
}

#[test]
fn test_return_qualifiers() {
    test("?a1@@YAXXZ", "void __cdecl a1(void)");
    test("?a2@@YAHXZ", "int __cdecl a2(void)");
    test("?a3@@YA?BHXZ", "int const __cdecl a3(void)");
    test("?a4@@YA?CHXZ", "int volatile __cdecl a4(void)");
    test("?a5@@YA?DHXZ", "int const volatile __cdecl a5(void)");
    test("?a6@@YAMXZ", "float __cdecl a6(void)");
    test("?b1@@YAPAHXZ", "int * __cdecl b1(void)");
    test("?b2@@YAPBDXZ", "char const * __cdecl b2(void)");
    test("?b3@@YAPAMXZ", "float * __cdecl b3(void)");
    test("?b4@@YAPBMXZ", "float const * __cdecl b4(void)");
    test("?b5@@YAPCMXZ", "float volatile * __cdecl b5(void)");
    test("?b6@@YAPDMXZ", "float const volatile * __cdecl b6(void)");
    test("?b7@@YAAAMXZ", "float & __cdecl b7(void)");
    test("?b8@@YAABMXZ", "float const & __cdecl b8(void)");
    test("?b9@@YAACMXZ", "float volatile & __cdecl b9(void)");
    test("?b10@@YAADMXZ", "float const volatile & __cdecl b10(void)");
    test("?b11@@YAPAPBDXZ", "char const ** __cdecl b11(void)");
    test("?c1@@YA?AVA@@XZ", "class A __cdecl c1(void)");
    test("?c2@@YA?BVA@@XZ", "class A const __cdecl c2(void)");
    test("?c3@@YA?CVA@@XZ", "class A volatile __cdecl c3(void)");
    test("?c4@@YA?DVA@@XZ", "class A const volatile __cdecl c4(void)");
    test("?c5@@YAPBVA@@XZ", "class A const * __cdecl c5(void)");
    test("?c6@@YAPCVA@@XZ", "class A volatile * __cdecl c6(void)");
    test(
        "?c7@@YAPDVA@@XZ",
        "class A const volatile * __cdecl c7(void)",
    );
    test("?c8@@YAAAVA@@XZ", "class A & __cdecl c8(void)");
    test("?c9@@YAABVA@@XZ", "class A const & __cdecl c9(void)");
    test("?c10@@YAACVA@@XZ", "class A volatile & __cdecl c10(void)");
    test(
        "?c11@@YAADVA@@XZ",
        "class A const volatile & __cdecl c11(void)",
    );
    test("?d1@@YA?AV?$B@H@@XZ", "class B<int> __cdecl d1(void)");
    test(
        "?d2@@YA?AV?$B@PBD@@XZ",
        "class B<char const *> __cdecl d2(void)",
    );
    test(
        "?d3@@YA?AV?$B@VA@@@@XZ",
        "class B<class A> __cdecl d3(void)",
    );
    test(
        "?d4@@YAPAV?$B@VA@@@@XZ",
        "class B<class A> * __cdecl d4(void)",
    );
    test(
        "?d5@@YAPBV?$B@VA@@@@XZ",
        "class B<class A> const * __cdecl d5(void)",
    );
    test(
        "?d6@@YAPCV?$B@VA@@@@XZ",
        "class B<class A> volatile * __cdecl d6(void)",
    );
    test(
        "?d7@@YAPDV?$B@VA@@@@XZ",
        "class B<class A> const volatile * __cdecl d7(void)",
    );
    test(
        "?d8@@YAAAV?$B@VA@@@@XZ",
        "class B<class A> & __cdecl d8(void)",
    );
    test(
        "?d9@@YAABV?$B@VA@@@@XZ",
        "class B<class A> const & __cdecl d9(void)",
    );
    test(
        "?d10@@YAACV?$B@VA@@@@XZ",
        "class B<class A> volatile & __cdecl d10(void)",
    );
    test(
        "?d11@@YAADV?$B@VA@@@@XZ",
        "class B<class A> const volatile & __cdecl d11(void)",
    );
    test("?e1@@YA?AW4Enum@@XZ", "Enum __cdecl e1(void)");
    test("?e2@@YA?BW4Enum@@XZ", "Enum const __cdecl e2(void)");
    test("?e3@@YAPAW4Enum@@XZ", "Enum * __cdecl e3(void)");
    test("?e4@@YAAAW4Enum@@XZ", "Enum & __cdecl e4(void)");
    test("?f1@@YA?AUS@@XZ", "struct S __cdecl f1(void)");
    test("?f2@@YA?BUS@@XZ", "struct S const __cdecl f2(void)");
    test("?f3@@YAPAUS@@XZ", "struct S * __cdecl f3(void)");
    test("?f4@@YAPBUS@@XZ", "struct S const * __cdecl f4(void)");
    test(
        "?f5@@YAPDUS@@XZ",
        "struct S const volatile * __cdecl f5(void)",
    );
    test("?f6@@YAAAUS@@XZ", "struct S & __cdecl f6(void)");
    test("?f7@@YAQAUS@@XZ", "struct S *const __cdecl f7(void)");
    test("?f8@@YAPQS@@HXZ", "int S::* __cdecl f8(void)");
    test("?f9@@YAQQS@@HXZ", "int S::*const __cdecl f9(void)");
    test("?f10@@YAPIQS@@HXZ", "int S::*__restrict __cdecl f10(void)");
    test(
        "?f11@@YAQIQS@@HXZ",
        "int S::*const __restrict __cdecl f11(void)",
    );
    test("?g1@@YAP6AHH@ZXZ", "int (__cdecl * __cdecl g1(void))(int)");
    test(
        "?g2@@YAQ6AHH@ZXZ",
        "int (__cdecl *const __cdecl g2(void))(int)",
    );
    test(
        "?g3@@YAPAP6AHH@ZXZ",
        "int (__cdecl ** __cdecl g3(void))(int)",
    );
    test(
        "?g4@@YAPBQ6AHH@ZXZ",
        "int (__cdecl *const * __cdecl g4(void))(int)",
    );
    test("?h1@@YAAIAHXZ", "int &__restrict __cdecl h1(void)");
}

#[test]
fn test_string_literals() {
    {
        let inputs: [&str; 256] = [
            "??_C@_01CNACBAHC@?$PP?$AA@",
            "??_C@_01DEBJCBDD@?$PO?$AA@",
            "??_C@_01BPDEHCPA@?$PN?$AA@",
            "??_C@_01GCPEDLB@?$PM?$AA@",
            "??_C@_01EJGONFHG@?$PL?$AA@",
            "??_C@_01FAHFOEDH@?z?$AA@",
            "??_C@_01HLFILHPE@?y?$AA@",
            "??_C@_01GCEDIGLF@?x?$AA@",
            "??_C@_01OFNLJKHK@?w?$AA@",
            "??_C@_01PMMAKLDL@?v?$AA@",
            "??_C@_01NHONPIPI@?u?$AA@",
            "??_C@_01MOPGMJLJ@?t?$AA@",
            "??_C@_01IBLHFPHO@?s?$AA@",
            "??_C@_01JIKMGODP@?r?$AA@",
            "??_C@_01LDIBDNPM@?q?$AA@",
            "??_C@_01KKJKAMLN@?p?$AA@",
            "??_C@_01GHMAACCD@?o?$AA@",
            "??_C@_01HONLDDGC@?n?$AA@",
            "??_C@_01FFPGGAKB@?m?$AA@",
            "??_C@_01EMONFBOA@?l?$AA@",
            "??_C@_01DKMMHCH@?k?$AA@",
            "??_C@_01BKLHPGGG@?j?$AA@",
            "??_C@_01DBJKKFKF@?i?$AA@",
            "??_C@_01CIIBJEOE@?h?$AA@",
            "??_C@_01KPBJIICL@?g?$AA@",
            "??_C@_01LGACLJGK@?f?$AA@",
            "??_C@_01JNCPOKKJ@?e?$AA@",
            "??_C@_01IEDENLOI@?d?$AA@",
            "??_C@_01MLHFENCP@?c?$AA@",
            "??_C@_01NCGOHMGO@?b?$AA@",
            "??_C@_01PJEDCPKN@?a?$AA@",
            "??_C@_01OAFIBOOM@?$OA?$AA@",
            "??_C@_01LIIGDENA@?$NP?$AA@",
            "??_C@_01KBJNAFJB@?$NO?$AA@",
            "??_C@_01IKLAFGFC@?$NN?$AA@",
            "??_C@_01JDKLGHBD@?$NM?$AA@",
            "??_C@_01NMOKPBNE@?$NL?$AA@",
            "??_C@_01MFPBMAJF@?Z?$AA@",
            "??_C@_01OONMJDFG@?Y?$AA@",
            "??_C@_01PHMHKCBH@?X?$AA@",
            "??_C@_01HAFPLONI@?W?$AA@",
            "??_C@_01GJEEIPJJ@?V?$AA@",
            "??_C@_01ECGJNMFK@?U?$AA@",
            "??_C@_01FLHCONBL@?T?$AA@",
            "??_C@_01BEDDHLNM@?S?$AA@",
            "??_C@_01NCIEKJN@?R?$AA@",
            "??_C@_01CGAFBJFO@?Q?$AA@",
            "??_C@_01DPBOCIBP@?P?$AA@",
            "??_C@_01PCEECGIB@?O?$AA@",
            "??_C@_01OLFPBHMA@?N?$AA@",
            "??_C@_01MAHCEEAD@?M?$AA@",
            "??_C@_01NJGJHFEC@?L?$AA@",
            "??_C@_01JGCIODIF@?K?$AA@",
            "??_C@_01IPDDNCME@?J?$AA@",
            "??_C@_01KEBOIBAH@?I?$AA@",
            "??_C@_01LNAFLAEG@?H?$AA@",
            "??_C@_01DKJNKMIJ@?G?$AA@",
            "??_C@_01CDIGJNMI@?F?$AA@",
            "??_C@_01IKLMOAL@?E?$AA@",
            "??_C@_01BBLAPPEK@?D?$AA@",
            "??_C@_01FOPBGJIN@?C?$AA@",
            "??_C@_01EHOKFIMM@?B?$AA@",
            "??_C@_01GMMHALAP@?A?$AA@",
            "??_C@_01HFNMDKEO@?$MA?$AA@",
            "??_C@_01NNHLFPHH@?$LP?$AA@",
            "??_C@_01MEGAGODG@?$LO?$AA@",
            "??_C@_01OPENDNPF@?$LN?$AA@",
            "??_C@_01PGFGAMLE@?$LM?$AA@",
            "??_C@_01LJBHJKHD@?$LL?$AA@",
            "??_C@_01KAAMKLDC@?$LK?$AA@",
            "??_C@_01ILCBPIPB@?$LJ?$AA@",
            "??_C@_01JCDKMJLA@?$LI?$AA@",
            "??_C@_01BFKCNFHP@?$LH?$AA@",
            "??_C@_01MLJOEDO@?$LG?$AA@",
            "??_C@_01CHJELHPN@?$LF?$AA@",
            "??_C@_01DOIPIGLM@?$LE?$AA@",
            "??_C@_01HBMOBAHL@?$LD?$AA@",
            "??_C@_01GINFCBDK@?$LC?$AA@",
            "??_C@_01EDPIHCPJ@?$LB?$AA@",
            "??_C@_01FKODEDLI@?$LA?$AA@",
            "??_C@_01JHLJENCG@?$KP?$AA@",
            "??_C@_01IOKCHMGH@?$KO?$AA@",
            "??_C@_01KFIPCPKE@?$KN?$AA@",
            "??_C@_01LMJEBOOF@?$KM?$AA@",
            "??_C@_01PDNFIICC@?$KL?$AA@",
            "??_C@_01OKMOLJGD@?$KK?$AA@",
            "??_C@_01MBODOKKA@?$KJ?$AA@",
            "??_C@_01NIPINLOB@?$KI?$AA@",
            "??_C@_01FPGAMHCO@?$KH?$AA@",
            "??_C@_01EGHLPGGP@?$KG?$AA@",
            "??_C@_01GNFGKFKM@?$KF?$AA@",
            "??_C@_01HEENJEON@?$KE?$AA@",
            "??_C@_01DLAMACCK@?$KD?$AA@",
            "??_C@_01CCBHDDGL@?$KC?$AA@",
            "??_C@_01JDKGAKI@?$KB?$AA@",
            "??_C@_01BACBFBOJ@?$KA?$AA@",
            "??_C@_01EIPPHLNF@?$JP?$AA@",
            "??_C@_01FBOEEKJE@?$JO?$AA@",
            "??_C@_01HKMJBJFH@?$JN?$AA@",
            "??_C@_01GDNCCIBG@?$JM?$AA@",
            "??_C@_01CMJDLONB@?$JL?$AA@",
            "??_C@_01DFIIIPJA@?$JK?$AA@",
            "??_C@_01BOKFNMFD@?$JJ?$AA@",
            "??_C@_01HLOONBC@?$JI?$AA@",
            "??_C@_01IACGPBNN@?$JH?$AA@",
            "??_C@_01JJDNMAJM@?$JG?$AA@",
            "??_C@_01LCBAJDFP@?$JF?$AA@",
            "??_C@_01KLALKCBO@?$JE?$AA@",
            "??_C@_01OEEKDENJ@?$JD?$AA@",
            "??_C@_01PNFBAFJI@?$JC?$AA@",
            "??_C@_01NGHMFGFL@?$JB?$AA@",
            "??_C@_01MPGHGHBK@?$JA?$AA@",
            "??_C@_01CDNGJIE@?$IP?$AA@",
            "??_C@_01BLCGFIMF@?$IO?$AA@",
            "??_C@_01DAALALAG@?$IN?$AA@",
            "??_C@_01CJBADKEH@?$IM?$AA@",
            "??_C@_01GGFBKMIA@?$IL?$AA@",
            "??_C@_01HPEKJNMB@?$IK?$AA@",
            "??_C@_01FEGHMOAC@?$IJ?$AA@",
            "??_C@_01ENHMPPED@?$II?$AA@",
            "??_C@_01MKOEODIM@?$IH?$AA@",
            "??_C@_01NDPPNCMN@?$IG?$AA@",
            "??_C@_01PINCIBAO@?$IF?$AA@",
            "??_C@_01OBMJLAEP@?$IE?$AA@",
            "??_C@_01KOIICGII@?$ID?$AA@",
            "??_C@_01LHJDBHMJ@?$IC?$AA@",
            "??_C@_01JMLOEEAK@?$IB?$AA@",
            "??_C@_01IFKFHFEL@?$IA?$AA@",
            "??_C@_01BGIBIIDJ@?$HP?$AA@",
            "??_C@_01PJKLJHI@?$HO?$AA@",
            "??_C@_01CELHOKLL@?$HN?$AA@",
            "??_C@_01DNKMNLPK@?$HM?$AA@",
            "??_C@_01HCONENDN@?$HL?$AA@",
            "??_C@_01GLPGHMHM@z?$AA@",
            "??_C@_01EANLCPLP@y?$AA@",
            "??_C@_01FJMABOPO@x?$AA@",
            "??_C@_01NOFIACDB@w?$AA@",
            "??_C@_01MHEDDDHA@v?$AA@",
            "??_C@_01OMGOGALD@u?$AA@",
            "??_C@_01PFHFFBPC@t?$AA@",
            "??_C@_01LKDEMHDF@s?$AA@",
            "??_C@_01KDCPPGHE@r?$AA@",
            "??_C@_01IIACKFLH@q?$AA@",
            "??_C@_01JBBJJEPG@p?$AA@",
            "??_C@_01FMEDJKGI@o?$AA@",
            "??_C@_01EFFIKLCJ@n?$AA@",
            "??_C@_01GOHFPIOK@m?$AA@",
            "??_C@_01HHGOMJKL@l?$AA@",
            "??_C@_01DICPFPGM@k?$AA@",
            "??_C@_01CBDEGOCN@j?$AA@",
            "??_C@_01KBJDNOO@i?$AA@",
            "??_C@_01BDACAMKP@h?$AA@",
            "??_C@_01JEJKBAGA@g?$AA@",
            "??_C@_01INIBCBCB@f?$AA@",
            "??_C@_01KGKMHCOC@e?$AA@",
            "??_C@_01LPLHEDKD@d?$AA@",
            "??_C@_01PAPGNFGE@c?$AA@",
            "??_C@_01OJONOECF@b?$AA@",
            "??_C@_01MCMALHOG@a?$AA@",
            "??_C@_01NLNLIGKH@?$GA?$AA@",
            "??_C@_01IDAFKMJL@_?$AA@",
            "??_C@_01JKBOJNNK@?$FO?$AA@",
            "??_C@_01LBDDMOBJ@?$FN?$AA@",
            "??_C@_01KICIPPFI@?2?$AA@",
            "??_C@_01OHGJGJJP@?$FL?$AA@",
            "??_C@_01POHCFINO@Z?$AA@",
            "??_C@_01NFFPALBN@Y?$AA@",
            "??_C@_01MMEEDKFM@X?$AA@",
            "??_C@_01ELNMCGJD@W?$AA@",
            "??_C@_01FCMHBHNC@V?$AA@",
            "??_C@_01HJOKEEBB@U?$AA@",
            "??_C@_01GAPBHFFA@T?$AA@",
            "??_C@_01CPLAODJH@S?$AA@",
            "??_C@_01DGKLNCNG@R?$AA@",
            "??_C@_01BNIGIBBF@Q?$AA@",
            "??_C@_01EJNLAFE@P?$AA@",
            "??_C@_01MJMHLOMK@O?$AA@",
            "??_C@_01NANMIPIL@N?$AA@",
            "??_C@_01PLPBNMEI@M?$AA@",
            "??_C@_01OCOKONAJ@L?$AA@",
            "??_C@_01KNKLHLMO@K?$AA@",
            "??_C@_01LELAEKIP@J?$AA@",
            "??_C@_01JPJNBJEM@I?$AA@",
            "??_C@_01IGIGCIAN@H?$AA@",
            "??_C@_01BBODEMC@G?$AA@",
            "??_C@_01BIAFAFID@F?$AA@",
            "??_C@_01DDCIFGEA@E?$AA@",
            "??_C@_01CKDDGHAB@D?$AA@",
            "??_C@_01GFHCPBMG@C?$AA@",
            "??_C@_01HMGJMAIH@B?$AA@",
            "??_C@_01FHEEJDEE@A?$AA@",
            "??_C@_01EOFPKCAF@?$EA?$AA@",
            "??_C@_01OGPIMHDM@?$DP?$AA@",
            "??_C@_01PPODPGHN@?$DO?$AA@",
            "??_C@_01NEMOKFLO@?$DN?$AA@",
            "??_C@_01MNNFJEPP@?$DM?$AA@",
            "??_C@_01ICJEACDI@?$DL?$AA@",
            "??_C@_01JLIPDDHJ@?3?$AA@",
            "??_C@_01LAKCGALK@9?$AA@",
            "??_C@_01KJLJFBPL@8?$AA@",
            "??_C@_01COCBENDE@7?$AA@",
            "??_C@_01DHDKHMHF@6?$AA@",
            "??_C@_01BMBHCPLG@5?$AA@",
            "??_C@_01FAMBOPH@4?$AA@",
            "??_C@_01EKENIIDA@3?$AA@",
            "??_C@_01FDFGLJHB@2?$AA@",
            "??_C@_01HIHLOKLC@1?$AA@",
            "??_C@_01GBGANLPD@0?$AA@",
            "??_C@_01KMDKNFGN@?1?$AA@",
            "??_C@_01LFCBOECM@?4?$AA@",
            "??_C@_01JOAMLHOP@?9?$AA@",
            "??_C@_01IHBHIGKO@?0?$AA@",
            "??_C@_01MIFGBAGJ@?$CL?$AA@",
            "??_C@_01NBENCBCI@?$CK?$AA@",
            "??_C@_01PKGAHCOL@?$CJ?$AA@",
            "??_C@_01ODHLEDKK@?$CI?$AA@",
            "??_C@_01GEODFPGF@?8?$AA@",
            "??_C@_01HNPIGOCE@?$CG?$AA@",
            "??_C@_01FGNFDNOH@?$CF?$AA@",
            "??_C@_01EPMOAMKG@$?$AA@",
            "??_C@_01IPJKGB@?$CD?$AA@",
            "??_C@_01BJJEKLCA@?$CC?$AA@",
            "??_C@_01DCLJPIOD@?$CB?$AA@",
            "??_C@_01CLKCMJKC@?5?$AA@",
            "??_C@_01HDHMODJO@?$BP?$AA@",
            "??_C@_01GKGHNCNP@?$BO?$AA@",
            "??_C@_01EBEKIBBM@?$BN?$AA@",
            "??_C@_01FIFBLAFN@?$BM?$AA@",
            "??_C@_01BHBACGJK@?$BL?$AA@",
            "??_C@_01OALBHNL@?$BK?$AA@",
            "??_C@_01CFCGEEBI@?$BJ?$AA@",
            "??_C@_01DMDNHFFJ@?$BI?$AA@",
            "??_C@_01LLKFGJJG@?$BH?$AA@",
            "??_C@_01KCLOFINH@?$BG?$AA@",
            "??_C@_01IJJDALBE@?$BF?$AA@",
            "??_C@_01JAIIDKFF@?$BE?$AA@",
            "??_C@_01NPMJKMJC@?$BD?$AA@",
            "??_C@_01MGNCJNND@?$BC?$AA@",
            "??_C@_01ONPPMOBA@?$BB?$AA@",
            "??_C@_01PEOEPPFB@?$BA?$AA@",
            "??_C@_01DJLOPBMP@?$AP?$AA@",
            "??_C@_01CAKFMAIO@?$AO?$AA@",
            "??_C@_01LIIJDEN@?$AN?$AA@",
            "??_C@_01BCJDKCAM@?$AM?$AA@",
            "??_C@_01FNNCDEML@?$AL?$AA@",
            "??_C@_01EEMJAFIK@?6?$AA@",
            "??_C@_01GPOEFGEJ@?7?$AA@",
            "??_C@_01HGPPGHAI@?$AI?$AA@",
            "??_C@_01PBGHHLMH@?$AH?$AA@",
            "??_C@_01OIHMEKIG@?$AG?$AA@",
            "??_C@_01MDFBBJEF@?$AF?$AA@",
            "??_C@_01NKEKCIAE@?$AE?$AA@",
            "??_C@_01JFALLOMD@?$AD?$AA@",
            "??_C@_01IMBAIPIC@?$AC?$AA@",
            "??_C@_01KHDNNMEB@?$AB?$AA@",
            "??_C@_01LOCGONAA@?$AA?$AA@",
        ];
        let outputs: [&str; 256] = [
            "\"\\xFF\"",
            "\"\\xFE\"",
            "\"\\xFD\"",
            "\"\\xFC\"",
            "\"\\xFB\"",
            "\"\\xFA\"",
            "\"\\xF9\"",
            "\"\\xF8\"",
            "\"\\xF7\"",
            "\"\\xF6\"",
            "\"\\xF5\"",
            "\"\\xF4\"",
            "\"\\xF3\"",
            "\"\\xF2\"",
            "\"\\xF1\"",
            "\"\\xF0\"",
            "\"\\xEF\"",
            "\"\\xEE\"",
            "\"\\xED\"",
            "\"\\xEC\"",
            "\"\\xEB\"",
            "\"\\xEA\"",
            "\"\\xE9\"",
            "\"\\xE8\"",
            "\"\\xE7\"",
            "\"\\xE6\"",
            "\"\\xE5\"",
            "\"\\xE4\"",
            "\"\\xE3\"",
            "\"\\xE2\"",
            "\"\\xE1\"",
            "\"\\xE0\"",
            "\"\\xDF\"",
            "\"\\xDE\"",
            "\"\\xDD\"",
            "\"\\xDC\"",
            "\"\\xDB\"",
            "\"\\xDA\"",
            "\"\\xD9\"",
            "\"\\xD8\"",
            "\"\\xD7\"",
            "\"\\xD6\"",
            "\"\\xD5\"",
            "\"\\xD4\"",
            "\"\\xD3\"",
            "\"\\xD2\"",
            "\"\\xD1\"",
            "\"\\xD0\"",
            "\"\\xCF\"",
            "\"\\xCE\"",
            "\"\\xCD\"",
            "\"\\xCC\"",
            "\"\\xCB\"",
            "\"\\xCA\"",
            "\"\\xC9\"",
            "\"\\xC8\"",
            "\"\\xC7\"",
            "\"\\xC6\"",
            "\"\\xC5\"",
            "\"\\xC4\"",
            "\"\\xC3\"",
            "\"\\xC2\"",
            "\"\\xC1\"",
            "\"\\xC0\"",
            "\"\\xBF\"",
            "\"\\xBE\"",
            "\"\\xBD\"",
            "\"\\xBC\"",
            "\"\\xBB\"",
            "\"\\xBA\"",
            "\"\\xB9\"",
            "\"\\xB8\"",
            "\"\\xB7\"",
            "\"\\xB6\"",
            "\"\\xB5\"",
            "\"\\xB4\"",
            "\"\\xB3\"",
            "\"\\xB2\"",
            "\"\\xB1\"",
            "\"\\xB0\"",
            "\"\\xAF\"",
            "\"\\xAE\"",
            "\"\\xAD\"",
            "\"\\xAC\"",
            "\"\\xAB\"",
            "\"\\xAA\"",
            "\"\\xA9\"",
            "\"\\xA8\"",
            "\"\\xA7\"",
            "\"\\xA6\"",
            "\"\\xA5\"",
            "\"\\xA4\"",
            "\"\\xA3\"",
            "\"\\xA2\"",
            "\"\\xA1\"",
            "\"\\xA0\"",
            "\"\\x9F\"",
            "\"\\x9E\"",
            "\"\\x9D\"",
            "\"\\x9C\"",
            "\"\\x9B\"",
            "\"\\x9A\"",
            "\"\\x99\"",
            "\"\\x98\"",
            "\"\\x97\"",
            "\"\\x96\"",
            "\"\\x95\"",
            "\"\\x94\"",
            "\"\\x93\"",
            "\"\\x92\"",
            "\"\\x91\"",
            "\"\\x90\"",
            "\"\\x8F\"",
            "\"\\x8E\"",
            "\"\\x8D\"",
            "\"\\x8C\"",
            "\"\\x8B\"",
            "\"\\x8A\"",
            "\"\\x89\"",
            "\"\\x88\"",
            "\"\\x87\"",
            "\"\\x86\"",
            "\"\\x85\"",
            "\"\\x84\"",
            "\"\\x83\"",
            "\"\\x82\"",
            "\"\\x81\"",
            "\"\\x80\"",
            "\"\\x7F\"",
            "\"~\"",
            "\"}\"",
            "\"|\"",
            "\"{\"",
            "\"z\"",
            "\"y\"",
            "\"x\"",
            "\"w\"",
            "\"v\"",
            "\"u\"",
            "\"t\"",
            "\"s\"",
            "\"r\"",
            "\"q\"",
            "\"p\"",
            "\"o\"",
            "\"n\"",
            "\"m\"",
            "\"l\"",
            "\"k\"",
            "\"j\"",
            "\"i\"",
            "\"h\"",
            "\"g\"",
            "\"f\"",
            "\"e\"",
            "\"d\"",
            "\"c\"",
            "\"b\"",
            "\"a\"",
            "\"`\"",
            "\"_\"",
            "\"^\"",
            "\"]\"",
            "\"\\\\\"",
            "\"[\"",
            "\"Z\"",
            "\"Y\"",
            "\"X\"",
            "\"W\"",
            "\"V\"",
            "\"U\"",
            "\"T\"",
            "\"S\"",
            "\"R\"",
            "\"Q\"",
            "\"P\"",
            "\"O\"",
            "\"N\"",
            "\"M\"",
            "\"L\"",
            "\"K\"",
            "\"J\"",
            "\"I\"",
            "\"H\"",
            "\"G\"",
            "\"F\"",
            "\"E\"",
            "\"D\"",
            "\"C\"",
            "\"B\"",
            "\"A\"",
            "\"@\"",
            "\"?\"",
            "\">\"",
            "\"=\"",
            "\"<\"",
            "\";\"",
            "\":\"",
            "\"9\"",
            "\"8\"",
            "\"7\"",
            "\"6\"",
            "\"5\"",
            "\"4\"",
            "\"3\"",
            "\"2\"",
            "\"1\"",
            "\"0\"",
            "\"/\"",
            "\".\"",
            "\"-\"",
            "\",\"",
            "\"+\"",
            "\"*\"",
            "\")\"",
            "\"(\"",
            "\"\\'\"",
            "\"&\"",
            "\"%\"",
            "\"$\"",
            "\"#\"",
            "\"\\\"\"",
            "\"!\"",
            "\" \"",
            "\"\\x1F\"",
            "\"\\x1E\"",
            "\"\\x1D\"",
            "\"\\x1C\"",
            "\"\\x1B\"",
            "\"\\x1A\"",
            "\"\\x19\"",
            "\"\\x18\"",
            "\"\\x17\"",
            "\"\\x16\"",
            "\"\\x15\"",
            "\"\\x14\"",
            "\"\\x13\"",
            "\"\\x12\"",
            "\"\\x11\"",
            "\"\\x10\"",
            "\"\\x0F\"",
            "\"\\x0E\"",
            "\"\\r\"",
            "\"\\f\"",
            "\"\\v\"",
            "\"\\n\"",
            "\"\\t\"",
            "\"\\b\"",
            "\"\\a\"",
            "\"\\x06\"",
            "\"\\x05\"",
            "\"\\x04\"",
            "\"\\x03\"",
            "\"\\x02\"",
            "\"\\x01\"",
            "u\"\"",
        ];

        for (input, output) in inputs.iter().zip(outputs) {
            test(input, output);
        }
    }

    {
        let inputs: [&str; 98] = [
            "??_C@_13KDLDGPGJ@?$AA?7?$AA?$AA@",
            "??_C@_13LBAGMAIH@?$AA?6?$AA?$AA@",
            "??_C@_13JLKKHOC@?$AA?$AL?$AA?$AA@",
            "??_C@_13HOIJIPNN@?$AA?5?$AA?$AA@",
            "??_C@_13MGDFOILI@?$AA?$CB?$AA?$AA@",
            "??_C@_13NEIAEHFG@?$AA?$CC?$AA?$AA@",
            "??_C@_13GMDMCADD@?$AA?$CD?$AA?$AA@",
            "??_C@_13PBOLBIIK@?$AA$?$AA?$AA@",
            "??_C@_13EJFHHPOP@?$AA?$CF?$AA?$AA@",
            "??_C@_13FLOCNAAB@?$AA?$CG?$AA?$AA@",
            "??_C@_13ODFOLHGE@?$AA?8?$AA?$AA@",
            "??_C@_13LLDNKHDC@?$AA?$CI?$AA?$AA@",
            "??_C@_13DIBMAFH@?$AA?$CJ?$AA?$AA@",
            "??_C@_13BBDEGPLJ@?$AA?$CK?$AA?$AA@",
            "??_C@_13KJIIAINM@?$AA?$CL?$AA?$AA@",
            "??_C@_13DEFPDAGF@?$AA?0?$AA?$AA@",
            "??_C@_13IMODFHAA@?$AA?9?$AA?$AA@",
            "??_C@_13JOFGPIOO@?$AA?4?$AA?$AA@",
            "??_C@_13CGOKJPIL@?$AA?1?$AA?$AA@",
            "??_C@_13COJANIEC@?$AA0?$AA?$AA@",
            "??_C@_13JGCMLPCH@?$AA1?$AA?$AA@",
            "??_C@_13IEJJBAMJ@?$AA2?$AA?$AA@",
            "??_C@_13DMCFHHKM@?$AA3?$AA?$AA@",
            "??_C@_13KBPCEPBF@?$AA4?$AA?$AA@",
            "??_C@_13BJEOCIHA@?$AA5?$AA?$AA@",
            "??_C@_13LPLIHJO@?$AA6?$AA?$AA@",
            "??_C@_13LDEHOAPL@?$AA7?$AA?$AA@",
            "??_C@_13OLCEPAKN@?$AA8?$AA?$AA@",
            "??_C@_13FDJIJHMI@?$AA9?$AA?$AA@",
            "??_C@_13EBCNDICG@?$AA?3?$AA?$AA@",
            "??_C@_13PJJBFPED@?$AA?$DL?$AA?$AA@",
            "??_C@_13GEEGGHPK@?$AA?$DM?$AA?$AA@",
            "??_C@_13NMPKAAJP@?$AA?$DN?$AA?$AA@",
            "??_C@_13MOEPKPHB@?$AA?$DO?$AA?$AA@",
            "??_C@_13HGPDMIBE@?$AA?$DP?$AA?$AA@",
            "??_C@_13EFKPHINO@?$AA?$EA?$AA?$AA@",
            "??_C@_13PNBDBPLL@?$AAA?$AA?$AA@",
            "??_C@_13OPKGLAFF@?$AAB?$AA?$AA@",
            "??_C@_13FHBKNHDA@?$AAC?$AA?$AA@",
            "??_C@_13MKMNOPIJ@?$AAD?$AA?$AA@",
            "??_C@_13HCHBIIOM@?$AAE?$AA?$AA@",
            "??_C@_13GAMECHAC@?$AAF?$AA?$AA@",
            "??_C@_13NIHIEAGH@?$AAG?$AA?$AA@",
            "??_C@_13IABLFADB@?$AAH?$AA?$AA@",
            "??_C@_13DIKHDHFE@?$AAI?$AA?$AA@",
            "??_C@_13CKBCJILK@?$AAJ?$AA?$AA@",
            "??_C@_13JCKOPPNP@?$AAK?$AA?$AA@",
            "??_C@_13PHJMHGG@?$AAL?$AA?$AA@",
            "??_C@_13LHMFKAAD@?$AAM?$AA?$AA@",
            "??_C@_13KFHAAPON@?$AAN?$AA?$AA@",
            "??_C@_13BNMMGIII@?$AAO?$AA?$AA@",
            "??_C@_13BFLGCPEB@?$AAP?$AA?$AA@",
            "??_C@_13KNAKEICE@?$AAQ?$AA?$AA@",
            "??_C@_13LPLPOHMK@?$AAR?$AA?$AA@",
            "??_C@_13HADIAKP@?$AAS?$AA?$AA@",
            "??_C@_13JKNELIBG@?$AAT?$AA?$AA@",
            "??_C@_13CCGINPHD@?$AAU?$AA?$AA@",
            "??_C@_13DANNHAJN@?$AAV?$AA?$AA@",
            "??_C@_13IIGBBHPI@?$AAW?$AA?$AA@",
            "??_C@_13NAACAHKO@?$AAX?$AA?$AA@",
            "??_C@_13GILOGAML@?$AAY?$AA?$AA@",
            "??_C@_13HKALMPCF@?$AAZ?$AA?$AA@",
            "??_C@_13MCLHKIEA@?$AA?$FL?$AA?$AA@",
            "??_C@_13FPGAJAPJ@?$AA?2?$AA?$AA@",
            "??_C@_13OHNMPHJM@?$AA?$FN?$AA?$AA@",
            "??_C@_13PFGJFIHC@?$AA?$FO?$AA?$AA@",
            "??_C@_13ENNFDPBH@?$AA_?$AA?$AA@",
            "??_C@_13OFJNNHOA@?$AA?$GA?$AA?$AA@",
            "??_C@_13FNCBLAIF@?$AAa?$AA?$AA@",
            "??_C@_13EPJEBPGL@?$AAb?$AA?$AA@",
            "??_C@_13PHCIHIAO@?$AAc?$AA?$AA@",
            "??_C@_13GKPPEALH@?$AAd?$AA?$AA@",
            "??_C@_13NCEDCHNC@?$AAe?$AA?$AA@",
            "??_C@_13MAPGIIDM@?$AAf?$AA?$AA@",
            "??_C@_13HIEKOPFJ@?$AAg?$AA?$AA@",
            "??_C@_13CACJPPAP@?$AAh?$AA?$AA@",
            "??_C@_13JIJFJIGK@?$AAi?$AA?$AA@",
            "??_C@_13IKCADHIE@?$AAj?$AA?$AA@",
            "??_C@_13DCJMFAOB@?$AAk?$AA?$AA@",
            "??_C@_13KPELGIFI@?$AAl?$AA?$AA@",
            "??_C@_13BHPHAPDN@?$AAm?$AA?$AA@",
            "??_C@_13FECKAND@?$AAn?$AA?$AA@",
            "??_C@_13LNPOMHLG@?$AAo?$AA?$AA@",
            "??_C@_13LFIEIAHP@?$AAp?$AA?$AA@",
            "??_C@_13NDIOHBK@?$AAq?$AA?$AA@",
            "??_C@_13BPINEIPE@?$AAr?$AA?$AA@",
            "??_C@_13KHDBCPJB@?$AAs?$AA?$AA@",
            "??_C@_13DKOGBHCI@?$AAt?$AA?$AA@",
            "??_C@_13ICFKHAEN@?$AAu?$AA?$AA@",
            "??_C@_13JAOPNPKD@?$AAv?$AA?$AA@",
            "??_C@_13CIFDLIMG@?$AAw?$AA?$AA@",
            "??_C@_13HADAKIJA@?$AAx?$AA?$AA@",
            "??_C@_13MIIMMPPF@?$AAy?$AA?$AA@",
            "??_C@_13NKDJGABL@?$AAz?$AA?$AA@",
            "??_C@_13GCIFAHHO@?$AA?$HL?$AA?$AA@",
            "??_C@_13PPFCDPMH@?$AA?$HM?$AA?$AA@",
            "??_C@_13EHOOFIKC@?$AA?$HN?$AA?$AA@",
            "??_C@_13FFFLPHEM@?$AA?$HO?$AA?$AA@",
        ];
        let outputs: [&str; 98] = [
            "\"\\t\"", "\"\\n\"", "\"\\v\"", "\" \"", "\"!\"", "\"\\\"\"", "\"#\"", "\"$\"",
            "\"%\"", "\"&\"", "\"\\'\"", "\"(\"", "\")\"", "\"*\"", "\"+\"", "\",\"", "\"-\"",
            "\".\"", "\"/\"", "\"0\"", "\"1\"", "\"2\"", "\"3\"", "\"4\"", "\"5\"", "\"6\"",
            "\"7\"", "\"8\"", "\"9\"", "\":\"", "\";\"", "\"<\"", "\"=\"", "\">\"", "\"?\"",
            "\"@\"", "\"A\"", "\"B\"", "\"C\"", "\"D\"", "\"E\"", "\"F\"", "\"G\"", "\"H\"",
            "\"I\"", "\"J\"", "\"K\"", "\"L\"", "\"M\"", "\"N\"", "\"O\"", "\"P\"", "\"Q\"",
            "\"R\"", "\"S\"", "\"T\"", "\"U\"", "\"V\"", "\"W\"", "\"X\"", "\"Y\"", "\"Z\"",
            "\"[\"", "\"\\\\\"", "\"]\"", "\"^\"", "\"_\"", "\"`\"", "\"a\"", "\"b\"", "\"c\"",
            "\"d\"", "\"e\"", "\"f\"", "\"g\"", "\"h\"", "\"i\"", "\"j\"", "\"k\"", "\"l\"",
            "\"m\"", "\"n\"", "\"o\"", "\"p\"", "\"q\"", "\"r\"", "\"s\"", "\"t\"", "\"u\"",
            "\"v\"", "\"w\"", "\"x\"", "\"y\"", "\"z\"", "\"{\"", "\"|\"", "\"}\"", "\"~\"",
        ];

        for (input, output) in inputs.iter().zip(outputs) {
            test(input, output);
        }
    }

    test(
        "??_C@_0CF@LABBIIMO@012345678901234567890123456789AB@",
        "\"012345678901234567890123456789AB\"...",
    );
    test("??_C@_1EK@KFPEBLPK@?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AAA?$AAB@", "L\"012345678901234567890123456789AB\"...");
    test("??_C@_13IIHIAFKH@?W?$PP?$AA?$AA@", "L\"\\xD7FF\"");
    test("??_C@_03IIHIAFKH@?$PP?W?$AA?$AA@", "u\"\\xD7FF\"");
    test("??_C@_02PCEFGMJL@hi?$AA@", "\"hi\"");
    test("??_C@_05OMLEGLOC@h?$AAi?$AA?$AA?$AA@", "u\"hi\"");
    test("??_C@_0EK@FEAOBHPP@o?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA@", "u\"o123456789012345\"...");
    test(
        "??_C@_0M@GFNAJIPG@h?$AA?$AA?$AAi?$AA?$AA?$AA?$AA?$AA?$AA?$AA@",
        "U\"hi\"",
    );
    test("??_C@_0JE@IMHFEDAA@0?$AA?$AA?$AA1?$AA?$AA?$AA2?$AA?$AA?$AA3?$AA?$AA?$AA4?$AA?$AA?$AA5?$AA?$AA?$AA6?$AA?$AA?$AA7?$AA?$AA?$AA@", "U\"01234567\"...");
    test(
        "??_C@_0CA@NMANGEKF@012345678901234567890123456789A?$AA@",
        "\"012345678901234567890123456789A\"",
    );
    test("??_C@_1EA@LJAFPILO@?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AAA?$AA?$AA@", "L\"012345678901234567890123456789A\"");
    test(
        "??_C@_0CA@NMANGEKF@012345678901234567890123456789A?$AA@",
        "\"012345678901234567890123456789A\"",
    );
    test("??_C@_0CA@NFEFHIFO@0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA?$AA?$AA@", "u\"012345678901234\"");
    test("??_C@_0CA@KFPHPCC@0?$AA?$AA?$AA1?$AA?$AA?$AA2?$AA?$AA?$AA3?$AA?$AA?$AA4?$AA?$AA?$AA5?$AA?$AA?$AA6?$AA?$AA?$AA?$AA?$AA?$AA?$AA@", "U\"0123456\"");
    test("??_C@_0CG@HJGBPLNO@l?$AAo?$AAo?$AAk?$AAA?$AAh?$AAe?$AAa?$AAd?$AAH?$AAa?$AAr?$AAd?$AAB?$AAr?$AAe?$AAa?$AAk?$AA?$AA?$AA@", "u\"lookAheadHardBreak\"");
    test("??_C@_0CG@HJGBPLNO@l?$AAo?$AAo?$AAk?$AAA?$AAh?$AAe?$AAa?$AAd?$AAH?$AAa?$AAr?$AAd?$AAB?$AAr?$AAe?$AA@", "u\"lookAheadHardBre\"...");
    test("??_C@_05LABPAAN@b?$AA?$AA?$AA?$AA?$AA@", "u\"b\\0\"");
    test("??_C@_0CC@MBPKDIAM@a?$AA?$AA?$AAb?$AA?$AA?$AAc?$AA?$AA?$AAd?$AA?$AA?$AAe?$AA?$AA?$AAf?$AA?$AA?$AAg?$AA?$AA?$AAh?$AA?$AA?$AA@", "u\"a\\0b\\0c\\0d\\0e\\0f\\0g\\0h\\0\"...");
    test(
        "??_C@_07LJGFEJEB@D3?$CC?$BB?$AA?$AA?$AA?$AA@)",
        "U\"\\x11223344\"",
    );
    test(
        "??_C@_0GAAAAAAAA@GPLEPFHO@01234567890123456789012345678901@",
        "\"01234567890123456789012345678901\"...",
    );
}

#[test]
fn test_template_callback() {
    test(
        "?callback_void@@3V?$C@$$A6AXXZ@@A",
        "class C<void __cdecl(void)> callback_void",
    );
    test(
        "?callback_void_volatile@@3V?$C@$$A6AXXZ@@C",
        "class C<void __cdecl(void)> volatile callback_void_volatile",
    );
    test(
        "?callback_int@@3V?$C@$$A6AHXZ@@A",
        "C<int __cdecl(void)> callback_int",
    );
    test(
        "?callback_Type@@3V?$C@$$A6A?AVType@@XZ@@A",
        "C<class Type __cdecl(void)> callback_Type",
    );
    test(
        "?callback_void_int@@3V?$C@$$A6AXH@Z@@A",
        "C<void __cdecl(int)> callback_void_int",
    );
    test(
        "?callback_int_int@@3V?$C@$$A6AHH@Z@@A",
        "C<int __cdecl(int)> callback_int_int",
    );
    test(
        "?callback_void_Type@@3V?$C@$$A6AXVType@@@Z@@A",
        "C<void __cdecl(class Type)> callback_void_Type",
    );
    test(
        "?foo@@YAXV?$C@$$A6AXXZ@@@Z",
        "void __cdecl foo(class C<void __cdecl(void)>)",
    );
    test(
        "?function@@YAXV?$C@$$A6AXXZ@@@Z",
        "void __cdecl function(class C<void __cdecl(void)>)",
    );
    test(
        "?function_pointer@@YAXV?$C@P6AXXZ@@@Z",
        "void __cdecl function_pointer(class C<void (__cdecl *)(void)>)",
    );
    test(
        "?member_pointer@@YAXV?$C@P8Z@@AEXXZ@@@Z",
        "void __cdecl member_pointer(class C<void (__thiscall Z::*)(void)>)",
    );
    test(
        "??$bar@P6AHH@Z@@YAXP6AHH@Z@Z",
        "void __cdecl bar<int (__cdecl *)(int)>(int (__cdecl *)(int))",
    );
    test(
        "??$WrapFnPtr@$1?VoidFn@@YAXXZ@@YAXXZ",
        "void __cdecl WrapFnPtr<&void __cdecl VoidFn(void)>(void)",
    );
    test(
        "??$WrapFnRef@$1?VoidFn@@YAXXZ@@YAXXZ",
        "void __cdecl WrapFnRef<&void __cdecl VoidFn(void)>(void)",
    );
    test(
        "??$WrapFnPtr@$1?VoidStaticMethod@Thing@@SAXXZ@@YAXXZ",
        "void __cdecl WrapFnPtr<&public: static void __cdecl Thing::VoidStaticMethod(void)>(void)",
    );
    test(
        "??$WrapFnRef@$1?VoidStaticMethod@Thing@@SAXXZ@@YAXXZ",
        "void __cdecl WrapFnRef<&public: static void __cdecl Thing::VoidStaticMethod(void)>(void)",
    );
}

#[test]
fn test_templates_memptrs_2() {
    test("?m@@3U?$J@UM@@$0A@@@A", "struct J<struct M, 0> m");
    test("?m2@@3U?$K@UM@@$0?0@@A", "struct K<struct M, -1> m2");
    test("?n@@3U?$J@UN@@$HA@@@A", "struct J<struct N, {0}> n");
    test("?n2@@3U?$K@UN@@$0?0@@A", "struct K<struct N, -1> n2");
    test("?o@@3U?$J@UO@@$IA@A@@@A", "struct J<struct O, {0, 0}> o");
    test("?o2@@3U?$K@UO@@$FA@?0@@A", "struct K<struct O, {0, -1}> o2");
    test(
        "?p@@3U?$J@UP@@$JA@A@?0@@A",
        "struct J<struct P, {0, 0, -1}> p",
    );
    test(
        "?p2@@3U?$K@UP@@$GA@A@?0@@A",
        "struct K<struct P, {0, 0, -1}> p2",
    );
    test("??0?$ClassTemplate@$J??_9MostGeneral@@$BA@AEA@M@3@@QAE@XZ", "__thiscall ClassTemplate<{[thunk]: __thiscall MostGeneral::`vcall'{0, {flat}}, 0, 12, 4}>::ClassTemplate<{[thunk]: __thiscall MostGeneral::`vcall'{0, {flat}}, 0, 12, 4}>(void)");
}

#[test]
fn test_templates_memptrs() {
    test("??$CallMethod@UC@NegativeNVOffset@@$I??_912@$BA@AEPPPPPPPM@A@@@YAXAAUC@NegativeNVOffset@@@Z", "void __cdecl CallMethod<struct NegativeNVOffset::C, {[thunk]: __thiscall NegativeNVOffset::C::`vcall'{0, {flat}}, 4294967292, 0}>(struct NegativeNVOffset::C &)");
    test(
        "??$CallMethod@UM@@$0A@@@YAXAAUM@@@Z",
        "void __cdecl CallMethod<struct M, 0>(struct M &)",
    );
    test("??$CallMethod@UM@@$H??_91@$BA@AEA@@@YAXAAUM@@@Z", "void __cdecl CallMethod<struct M, {[thunk]: __thiscall M::`vcall'{0, {flat}}, 0}>(struct M &)");
    test(
        "??$CallMethod@UM@@$H?f@1@QAEXXZA@@@YAXAAUM@@@Z",
        "void __cdecl CallMethod<struct M, {public: void __thiscall M::f(void), 0}>(struct M &)",
    );
    test("??$CallMethod@UO@@$H??_91@$BA@AE3@@YAXAAUO@@@Z", "void __cdecl CallMethod<struct O, {[thunk]: __thiscall O::`vcall'{0, {flat}}, 4}>(struct O &)");
    test(
        "??$CallMethod@US@@$0A@@@YAXAAUS@@@Z",
        "void __cdecl CallMethod<struct S, 0>(struct S &)",
    );
    test(
        "??$CallMethod@US@@$1??_91@$BA@AE@@YAXAAUS@@@Z",
        "void __cdecl CallMethod<struct S, &[thunk]: __thiscall S::`vcall'{0, {flat}}>(struct S &)",
    );
    test(
        "??$CallMethod@US@@$1?f@1@QAEXXZ@@YAXAAUS@@@Z",
        "void __cdecl CallMethod<struct S, &public: void __thiscall S::f(void)>(struct S &)",
    );
    test(
        "??$CallMethod@UU@@$0A@@@YAXAAUU@@@Z",
        "void __cdecl CallMethod<struct U, 0>(struct U &)",
    );
    test("??$CallMethod@UU@@$J??_91@$BA@AEA@A@A@@@YAXAAUU@@@Z", "void __cdecl CallMethod<struct U, {[thunk]: __thiscall U::`vcall'{0, {flat}}, 0, 0, 0}>(struct U &)");
    test("??$CallMethod@UU@@$J?f@1@QAEXXZA@A@A@@@YAXAAUU@@@Z", "void __cdecl CallMethod<struct U, {public: void __thiscall U::f(void), 0, 0, 0}>(struct U &)");
    test(
        "??$CallMethod@UV@@$0A@@@YAXAAUV@@@Z",
        "void __cdecl CallMethod<struct V, 0>(struct V &)",
    );
    test("??$CallMethod@UV@@$I??_91@$BA@AEA@A@@@YAXAAUV@@@Z", "void __cdecl CallMethod<struct V, {[thunk]: __thiscall V::`vcall'{0, {flat}}, 0, 0}>(struct V &)");
    test(
        "??$CallMethod@UV@@$I?f@1@QAEXXZA@A@@@YAXAAUV@@@Z",
        "void __cdecl CallMethod<struct V, {public: void __thiscall V::f(void), 0, 0}>(struct V &)",
    );
    test(
        "??$ReadField@UA@@$0?0@@YAHAAUA@@@Z",
        "int __cdecl ReadField<struct A, -1>(struct A &)",
    );
    test(
        "??$ReadField@UA@@$0A@@@YAHAAUA@@@Z",
        "int __cdecl ReadField<struct A, 0>(struct A &)",
    );
    test(
        "??$ReadField@UI@@$03@@YAHAAUI@@@Z",
        "int __cdecl ReadField<struct I, 4>(struct I &)",
    );
    test(
        "??$ReadField@UI@@$0A@@@YAHAAUI@@@Z",
        "int __cdecl ReadField<struct I, 0>(struct I &)",
    );
    test(
        "??$ReadField@UM@@$0A@@@YAHAAUM@@@Z",
        "int __cdecl ReadField<struct M, 0>(struct M &)",
    );
    test(
        "??$ReadField@UM@@$0BA@@@YAHAAUM@@@Z",
        "int __cdecl ReadField<struct M, 16>(struct M &)",
    );
    test(
        "??$ReadField@UM@@$0M@@@YAHAAUM@@@Z",
        "int __cdecl ReadField<struct M, 12>(struct M &)",
    );
    test(
        "??$ReadField@US@@$03@@YAHAAUS@@@Z",
        "int __cdecl ReadField<struct S, 4>(struct S &)",
    );
    test(
        "??$ReadField@US@@$07@@YAHAAUS@@@Z",
        "int __cdecl ReadField<struct S, 8>(struct S &)",
    );
    test(
        "??$ReadField@US@@$0A@@@YAHAAUS@@@Z",
        "int __cdecl ReadField<struct S, 0>(struct S &)",
    );
    test(
        "??$ReadField@UU@@$0A@@@YAHAAUU@@@Z",
        "int __cdecl ReadField<struct U, 0>(struct U &)",
    );
    test(
        "??$ReadField@UU@@$G3A@A@@@YAHAAUU@@@Z",
        "int __cdecl ReadField<struct U, {4, 0, 0}>(struct U &)",
    );
    test(
        "??$ReadField@UU@@$G7A@A@@@YAHAAUU@@@Z",
        "int __cdecl ReadField<struct U, {8, 0, 0}>(struct U &)",
    );
    test(
        "??$ReadField@UV@@$0A@@@YAHAAUV@@@Z",
        "int __cdecl ReadField<struct V, 0>(struct V &)",
    );
    test(
        "??$ReadField@UV@@$F7A@@@YAHAAUV@@@Z",
        "int __cdecl ReadField<struct V, {8, 0}>(struct V &)",
    );
    test(
        "??$ReadField@UV@@$FM@A@@@YAHAAUV@@@Z",
        "int __cdecl ReadField<struct V, {12, 0}>(struct V &)",
    );
    test(
        "?Q@@3$$QEAP8Foo@@EAAXXZEA",
        "void (__cdecl Foo::*&&Q)(void)",
    );
}

#[test]
fn test_templates() {
    test("?f@@3V?$C@H@@A", "class C<int> f");
    test(
        "??0?$Class@VTypename@@@@QAE@XZ",
        "__thiscall Class<class Typename>::Class<class Typename>(void)",
    );
    test(
        "??0?$Class@VTypename@@@@QEAA@XZ",
        "__cdecl Class<class Typename>::Class<class Typename>(void)",
    );
    test(
        "??0?$Class@$$CBVTypename@@@@QAE@XZ",
        "__thiscall Class<class Typename const>::Class<class Typename const>(void)",
    );
    test(
        "??0?$Class@$$CBVTypename@@@@QEAA@XZ",
        "__cdecl Class<class Typename const>::Class<class Typename const>(void)",
    );
    test(
        "??0?$Class@$$CCVTypename@@@@QAE@XZ",
        "__thiscall Class<class Typename volatile>::Class<class Typename volatile>(void)",
    );
    test(
        "??0?$Class@$$CCVTypename@@@@QEAA@XZ",
        "__cdecl Class<class Typename volatile>::Class<class Typename volatile>(void)",
    );
    test("??0?$Class@$$CDVTypename@@@@QAE@XZ", "__thiscall Class<class Typename const volatile>::Class<class Typename const volatile>(void)");
    test(
        "??0?$Class@$$CDVTypename@@@@QEAA@XZ",
        "__cdecl Class<class Typename const volatile>::Class<class Typename const volatile>(void)",
    );
    test(
        "??0?$Class@V?$Nested@VTypename@@@@@@QAE@XZ",
        "__thiscall Class<class Nested<class Typename>>::Class<class Nested<class Typename>>(void)",
    );
    test(
        "??0?$Class@V?$Nested@VTypename@@@@@@QEAA@XZ",
        "__cdecl Class<class Nested<class Typename>>::Class<class Nested<class Typename>>(void)",
    );
    test(
        "??0?$Class@QAH@@QAE@XZ",
        "__thiscall Class<int *const>::Class<int *const>(void)",
    );
    test(
        "??0?$Class@QEAH@@QEAA@XZ",
        "__cdecl Class<int *const>::Class<int *const>(void)",
    );
    test(
        "??0?$Class@$$A6AHXZ@@QAE@XZ",
        "__thiscall Class<int __cdecl(void)>::Class<int __cdecl(void)>(void)",
    );
    test(
        "??0?$Class@$$A6AHXZ@@QEAA@XZ",
        "__cdecl Class<int __cdecl(void)>::Class<int __cdecl(void)>(void)",
    );
    test(
        "??0?$Class@$$BY0A@H@@QAE@XZ",
        "__thiscall Class<int[]>::Class<int[]>(void)",
    );
    test(
        "??0?$Class@$$BY0A@H@@QEAA@XZ",
        "__cdecl Class<int[]>::Class<int[]>(void)",
    );
    test(
        "??0?$Class@$$BY04H@@QAE@XZ",
        "__thiscall Class<int[5]>::Class<int[5]>(void)",
    );
    test(
        "??0?$Class@$$BY04H@@QEAA@XZ",
        "__cdecl Class<int[5]>::Class<int[5]>(void)",
    );
    test(
        "??0?$Class@$$BY04$$CBH@@QAE@XZ",
        "__thiscall Class<int const[5]>::Class<int const[5]>(void)",
    );
    test(
        "??0?$Class@$$BY04$$CBH@@QEAA@XZ",
        "__cdecl Class<int const[5]>::Class<int const[5]>(void)",
    );
    test(
        "??0?$Class@$$BY04QAH@@QAE@XZ",
        "__thiscall Class<int *const[5]>::Class<int *const[5]>(void)",
    );
    test(
        "??0?$Class@$$BY04QEAH@@QEAA@XZ",
        "__cdecl Class<int *const[5]>::Class<int *const[5]>(void)",
    );
    test(
        "??0?$BoolTemplate@$0A@@@QAE@XZ",
        "__thiscall BoolTemplate<0>::BoolTemplate<0>(void)",
    );
    test(
        "??0?$BoolTemplate@$0A@@@QEAA@XZ",
        "__cdecl BoolTemplate<0>::BoolTemplate<0>(void)",
    );
    test(
        "??0?$BoolTemplate@$00@@QAE@XZ",
        "__thiscall BoolTemplate<1>::BoolTemplate<1>(void)",
    );
    test(
        "??0?$BoolTemplate@$00@@QEAA@XZ",
        "__cdecl BoolTemplate<1>::BoolTemplate<1>(void)",
    );
    test(
        "??$Foo@H@?$BoolTemplate@$00@@QAEXH@Z",
        "void __thiscall BoolTemplate<1>::Foo<int>(int)",
    );
    test(
        "??$Foo@H@?$BoolTemplate@$00@@QEAAXH@Z",
        "void __cdecl BoolTemplate<1>::Foo<int>(int)",
    );
    test(
        "??0?$IntTemplate@$0A@@@QAE@XZ",
        "__thiscall IntTemplate<0>::IntTemplate<0>(void)",
    );
    test(
        "??0?$IntTemplate@$0A@@@QEAA@XZ",
        "__cdecl IntTemplate<0>::IntTemplate<0>(void)",
    );
    test(
        "??0?$IntTemplate@$04@@QAE@XZ",
        "__thiscall IntTemplate<5>::IntTemplate<5>(void)",
    );
    test(
        "??0?$IntTemplate@$04@@QEAA@XZ",
        "__cdecl IntTemplate<5>::IntTemplate<5>(void)",
    );
    test(
        "??0?$IntTemplate@$0L@@@QAE@XZ",
        "__thiscall IntTemplate<11>::IntTemplate<11>(void)",
    );
    test(
        "??0?$IntTemplate@$0L@@@QEAA@XZ",
        "__cdecl IntTemplate<11>::IntTemplate<11>(void)",
    );
    test(
        "??0?$IntTemplate@$0BAA@@@QAE@XZ",
        "__thiscall IntTemplate<256>::IntTemplate<256>(void)",
    );
    test(
        "??0?$IntTemplate@$0BAA@@@QEAA@XZ",
        "__cdecl IntTemplate<256>::IntTemplate<256>(void)",
    );
    test(
        "??0?$IntTemplate@$0CAB@@@QAE@XZ",
        "__thiscall IntTemplate<513>::IntTemplate<513>(void)",
    );
    test(
        "??0?$IntTemplate@$0CAB@@@QEAA@XZ",
        "__cdecl IntTemplate<513>::IntTemplate<513>(void)",
    );
    test(
        "??0?$IntTemplate@$0EAC@@@QAE@XZ",
        "__thiscall IntTemplate<1026>::IntTemplate<1026>(void)",
    );
    test(
        "??0?$IntTemplate@$0EAC@@@QEAA@XZ",
        "__cdecl IntTemplate<1026>::IntTemplate<1026>(void)",
    );
    test(
        "??0?$IntTemplate@$0PPPP@@@QAE@XZ",
        "__thiscall IntTemplate<65535>::IntTemplate<65535>(void)",
    );
    test(
        "??0?$IntTemplate@$0PPPP@@@QEAA@XZ",
        "__cdecl IntTemplate<65535>::IntTemplate<65535>(void)",
    );
    test(
        "??0?$IntTemplate@$0?0@@QAE@XZ",
        "__thiscall IntTemplate<-1>::IntTemplate<-1>(void)",
    );
    test(
        "??0?$IntTemplate@$0?0@@QEAA@XZ",
        "__cdecl IntTemplate<-1>::IntTemplate<-1>(void)",
    );
    test(
        "??0?$IntTemplate@$0?8@@QAE@XZ",
        "__thiscall IntTemplate<-9>::IntTemplate<-9>(void)",
    );
    test(
        "??0?$IntTemplate@$0?8@@QEAA@XZ",
        "__cdecl IntTemplate<-9>::IntTemplate<-9>(void)",
    );
    test(
        "??0?$IntTemplate@$0?9@@QAE@XZ",
        "__thiscall IntTemplate<-10>::IntTemplate<-10>(void)",
    );
    test(
        "??0?$IntTemplate@$0?9@@QEAA@XZ",
        "__cdecl IntTemplate<-10>::IntTemplate<-10>(void)",
    );
    test(
        "??0?$IntTemplate@$0?L@@@QAE@XZ",
        "__thiscall IntTemplate<-11>::IntTemplate<-11>(void)",
    );
    test(
        "??0?$IntTemplate@$0?L@@@QEAA@XZ",
        "__cdecl IntTemplate<-11>::IntTemplate<-11>(void)",
    );
    test(
        "??0?$UnsignedIntTemplate@$0PPPPPPPP@@@QAE@XZ",
        "__thiscall UnsignedIntTemplate<4294967295>::UnsignedIntTemplate<4294967295>(void)",
    );
    test(
        "??0?$UnsignedIntTemplate@$0PPPPPPPP@@@QEAA@XZ",
        "__cdecl UnsignedIntTemplate<4294967295>::UnsignedIntTemplate<4294967295>(void)",
    );
    test("??0?$LongLongTemplate@$0?IAAAAAAAAAAAAAAA@@@QAE@XZ", "__thiscall LongLongTemplate<-9223372036854775808>::LongLongTemplate<-9223372036854775808>(void)");
    test("??0?$LongLongTemplate@$0?IAAAAAAAAAAAAAAA@@@QEAA@XZ", "__cdecl LongLongTemplate<-9223372036854775808>::LongLongTemplate<-9223372036854775808>(void)");
    test("??0?$LongLongTemplate@$0HPPPPPPPPPPPPPPP@@@QAE@XZ", "__thiscall LongLongTemplate<9223372036854775807>::LongLongTemplate<9223372036854775807>(void)");
    test("??0?$LongLongTemplate@$0HPPPPPPPPPPPPPPP@@@QEAA@XZ", "__cdecl LongLongTemplate<9223372036854775807>::LongLongTemplate<9223372036854775807>(void)");
    test(
        "??0?$UnsignedLongLongTemplate@$0?0@@QAE@XZ",
        "__thiscall UnsignedLongLongTemplate<-1>::UnsignedLongLongTemplate<-1>(void)",
    );
    test(
        "??0?$UnsignedLongLongTemplate@$0?0@@QEAA@XZ",
        "__cdecl UnsignedLongLongTemplate<-1>::UnsignedLongLongTemplate<-1>(void)",
    );
    test(
        "??$foo@H@space@@YAABHABH@Z",
        "int const & __cdecl space::foo<int>(int const &)",
    );
    test(
        "??$foo@H@space@@YAAEBHAEBH@Z",
        "int const & __cdecl space::foo<int>(int const &)",
    );
    test(
        "??$FunctionPointerTemplate@$1?spam@@YAXXZ@@YAXXZ",
        "void __cdecl FunctionPointerTemplate<&void __cdecl spam(void)>(void)",
    );
    test("??$variadic_fn_template@HHHH@@YAXABH000@Z", "void __cdecl variadic_fn_template<int, int, int, int>(int const &, int const &, int const &, int const &)");
    test("??$variadic_fn_template@HHD$$BY01D@@YAXABH0ABDAAY01$$CBD@Z", "void __cdecl variadic_fn_template<int, int, char, char[2]>(int const &, int const &, char const &, char const (&)[2]");
    test(
        "??0?$VariadicClass@HD_N@@QAE@XZ",
        "__thiscall VariadicClass<int, char, bool>::VariadicClass<int, char, bool>(void)",
    );
    test(
        "??0?$VariadicClass@_NDH@@QAE@XZ",
        "__thiscall VariadicClass<bool, char, int>::VariadicClass<bool, char, int>(void)",
    );
    test("?template_template_fun@@YAXU?$Type@U?$Thing@USecond@@$00@@USecond@@@@@Z", "void __cdecl template_template_fun(struct Type<struct Thing<struct Second, 1>, struct Second>)");
    test("??$template_template_specialization@$$A6AXU?$Type@U?$Thing@USecond@@$00@@USecond@@@@@Z@@YAXXZ", "void __cdecl template_template_specialization<void __cdecl(struct Type<struct Thing<struct Second, 1>, struct Second>)>(void)");
    test("?f@@YAXU?$S1@$0A@@@@Z", "void __cdecl f(struct S1<0>)");
    test(
        "?recref@@YAXU?$type1@$E?inst@@3Urecord@@B@@@Z",
        "void __cdecl recref(struct type1<struct record const inst>)",
    );
    test("?fun@@YAXU?$UUIDType1@Uuuid@@$1?_GUID_12345678_1234_1234_1234_1234567890ab@@3U__s_GUID@@B@@@Z", "void __cdecl fun(struct UUIDType1<struct uuid, &struct __s_GUID const _GUID_12345678_1234_1234_1234_1234567890ab>)");
    test("?fun@@YAXU?$UUIDType2@Uuuid@@$E?_GUID_12345678_1234_1234_1234_1234567890ab@@3U__s_GUID@@B@@@Z", "void __cdecl fun(struct UUIDType2<struct uuid, struct __s_GUID const _GUID_12345678_1234_1234_1234_1234567890ab>)");
    test(
        "?FunctionDefinedWithInjectedName@@YAXU?$TypeWithFriendDefinition@H@@@Z",
        "void __cdecl FunctionDefinedWithInjectedName(struct TypeWithFriendDefinition<int>)",
    );
    test("?bar@?$UUIDType4@$1?_GUID_12345678_1234_1234_1234_1234567890ab@@3U__s_GUID@@B@@QAEXXZ", "void __thiscall UUIDType4<&struct __s_GUID const _GUID_12345678_1234_1234_1234_1234567890ab>::bar(void)");
    test(
        "??$f@US@@$1?g@1@QEAAXXZ@@YAXXZ",
        "void __cdecl f<struct S, &public: void __cdecl S::g(void)>(void)",
    );
    test(
        "??$?0N@?$Foo@H@@QEAA@N@Z",
        "__cdecl Foo<int>::Foo<int><double>(double)",
    );
}

#[test]
fn test_thunks() {
    test(
        "?f@C@@WBA@EAAHXZ",
        "[thunk]: public: virtual int __cdecl C::f`adjustor{16}'(void)",
    );
    test("??_EDerived@@$4PPPPPPPM@A@EAAPEAXI@Z", "[thunk]: public: virtual void * __cdecl Derived::`vector deleting dtor'`vtordisp{-4, 0}'(unsigned int)");
    test(
        "?f@A@simple@@$R477PPPPPPPM@7AEXXZ",
        "[thunk]: public: virtual void __thiscall simple::A::f`vtordispex{8, 8, -4, 8}'(void)",
    );
    test(
        "??_9Base@@$B7AA",
        "[thunk]: __cdecl Base::`vcall'{8, {flat}}",
    );
}

#[test]
fn test_windows() {
    test("?bar@Foo@@SGXXZ", "static void __stdcall Foo::bar(void)");
    test("?bar@Foo@@QAGXXZ", "void __stdcall Foo::bar(void)");
    test("?f2@@YIXXZ", "void __fastcall f2(void)");
    test("?f1@@YGXXZ", "void __stdcall f1(void)");
    test("?f5@@YCXXZ", "void __pascal f5(void)");
}

#[test]
fn test_no_this_type() {
    let test_option = |mangled_name: &str, demangled_name: &str| {
        do_test(mangled_name, demangled_name, false, Flags::NO_THISTYPE);
    };

    test_option(
        "?world@hello@@QEBAXXZ",
        "public: void __cdecl hello::world(void)",
    );
    test_option(
        "?world@hello@@QECAXXZ",
        "public: void __cdecl hello::world(void)",
    );
    test_option(
        "?world@hello@@QEIAAXXZ",
        "public: void __cdecl hello::world(void)",
    );
    test_option(
        "?world@hello@@QEFAAXXZ",
        "public: void __cdecl hello::world(void)",
    );
    test_option(
        "?a@FTypeWithQuals@@3U?$S@$$A8@@BAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::a",
    );
    test_option(
        "?b@FTypeWithQuals@@3U?$S@$$A8@@CAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::b",
    );
    test_option(
        "?c@FTypeWithQuals@@3U?$S@$$A8@@IAAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::c",
    );
    test_option(
        "?d@FTypeWithQuals@@3U?$S@$$A8@@GBAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::d",
    );
    test_option(
        "?e@FTypeWithQuals@@3U?$S@$$A8@@GCAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::e",
    );
    test_option(
        "?f@FTypeWithQuals@@3U?$S@$$A8@@IGAAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::f",
    );
    test_option(
        "?g@FTypeWithQuals@@3U?$S@$$A8@@HBAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::g",
    );
    test_option(
        "?h@FTypeWithQuals@@3U?$S@$$A8@@HCAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::h",
    );
    test_option(
        "?i@FTypeWithQuals@@3U?$S@$$A8@@IHAAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::i",
    );
    test_option(
        "?j@FTypeWithQuals@@3U?$S@$$A6AHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::j",
    );
    test_option(
        "?k@FTypeWithQuals@@3U?$S@$$A8@@GAAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::k",
    );
    test_option(
        "?l@FTypeWithQuals@@3U?$S@$$A8@@HAAHXZ@1@A",
        "struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::l",
    );
}

#[test]
fn test_no_leading_underscores() {
    let test_option = |mangled_name: &str, demangled_name: &str| {
        do_test(
            mangled_name,
            demangled_name,
            false,
            Flags::NO_LEADING_UNDERSCORES,
        );
    };

    test_option(
        "?unaligned_foo5@@YAXPIFAH@Z",
        "void cdecl unaligned_foo5(int unaligned *restrict)",
    );
    test_option("?beta@@YI_N_J_W@Z", "bool fastcall beta(__int64, wchar_t)");
    test_option("?f5@@YCXXZ", "void pascal f5(void)");
    test_option(
        "?j@@3P6GHCE@ZA",
        "int (stdcall *j)(signed char, unsigned char)",
    );
    test_option(
        "?mbb@S@@QAEX_N0@Z",
        "public: void thiscall S::mbb(bool, bool)",
    );
    test_option("?vector_func@@YQXXZ", "void vectorcall vector_func(void)");
}

#[test]
fn test_no_ms_keywords() {
    let test_option = |mangled_name: &str, demangled_name: &str| {
        do_test(mangled_name, demangled_name, false, Flags::NO_MS_KEYWORDS);
    };

    test_option("?unaligned_foo5@@YAXPIFAH@Z", "void unaligned_foo5(int *)");
    test_option("?beta@@YI_N_J_W@Z", "bool beta(__int64, wchar_t)");
    test_option("?f5@@YCXXZ", "void f5(void)");
    test_option("?j@@3P6GHCE@ZA", "int (*j)(signed char, unsigned char)");
    test_option("?mbb@S@@QAEX_N0@Z", "public: void S::mbb(bool, bool)");
    test_option("?vector_func@@YQXXZ", "void vector_func(void)");
}

#[test]
fn test_name_only() {
    let test_option = |mangled_name: &str, demangled_name: &str| {
        do_test(mangled_name, demangled_name, false, Flags::NAME_ONLY);
    };

    test_option("?foo@@YAXI@Z", "foo");
    test_option("?foobarbazqux@NB@PR13207@@YAXV?$Y@VX@NB@PR13207@@@12@V?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NA@2@V412@2V?$Y@V?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NB@PR13207@@@52@@Z", "PR13207::NB::foobarbazqux");
    test_option("??$f@US@@$1?g@1@QEAAXXZ@@YAXXZ", "f<S, &S::g>");
    test_option("?foo_sad@@YAXSEAD@Z", "foo_sad");
    test_option("??$WrapFnPtr@$1?VoidFn@@YAXXZ@@YAXXZ", "WrapFnPtr<&VoidFn>");
    test_option("??$CallMethod@UM@@$0A@@@YAXAAUM@@@Z", "CallMethod<M, 0>");
    test_option("?fun@@YAXU?$UUIDType1@Uuuid@@$1?_GUID_12345678_1234_1234_1234_1234567890ab@@3U__s_GUID@@B@@@Z", "fun");
    test_option("?abc_foo@@YA?AV?$A@DV?$B@D@N@@V?$C@D@2@@N@@XZ", "abc_foo");
    test_option("?f2@@YA?BUS@@XZ", "f2");
    test_option("??Hfoo@@QAEHH@Z", "foo::operator+");
    test_option("?M@?1??L@@YAHXZ@4HA", "`L'::`2'::M");
    test_option("?h2@@3QBHB", "h2");
    test_option(
        "??$Foo@H@?$BoolTemplate@$00@@QEAAXH@Z",
        "BoolTemplate<1>::Foo<int>",
    );
    test_option(
        "??0?$IntTemplate@$0L@@@QAE@XZ",
        "IntTemplate<11>::IntTemplate<11>",
    );
    test_option("??_V@YAXPEAXAEAVklass@@@Z", "operator delete[]");
    test_option("??1klass@@QEAA@XZ", "klass::~klass");
    test_option(
        "?d@FTypeWithQuals@@3U?$S@$$A8@@GBAHXZ@1@A",
        "FTypeWithQuals::d",
    );
    test_option("??_7Base@@6B@", "Base::`vftable'");
    test_option("?h1@@YAAIAHXZ", "h1");
    test_option("?x@ns@@3PEAV?$klass@HH@1@EA", "ns::x");
    test_option(
        "??2OverloadedNewDelete@@SAPAXI@Z",
        "OverloadedNewDelete::operator new",
    );
    test_option("?x@@YAXMH@Z", "x");
    test_option(
        "?f@UnnamedType@@YAXQAPAU<unnamed-type-T1>@S@1@@Z",
        "UnnamedType::f",
    );
    test_option(
        "?spam@NB@PR13207@@YAXV?$Y@VX@NA@PR13207@@@NA@2@@Z",
        "PR13207::NB::spam",
    );
    test_option("??__FFoo@@YAXXZ", "`dynamic atexit destructor for 'Foo''");
    test_option("?delta@@YAXQAHABJ@Z", "delta");
    test_option(
        "?foo@NC@PR13207@@YAXV?$Y@VX@NB@PR13207@@@12@@Z",
        "PR13207::NC::foo",
    );
    test_option(
        "?lambda@?1??define_lambda@@YAHXZ@4V<lambda_1>@?0??1@YAHXZ@A",
        "`define_lambda'::`2'::lambda",
    );
    test_option("?X@?$C@H@C@0@2HB", "X::C::C<int>::X");
    test_option("?d5@@YAPBV?$B@VA@@@@XZ", "d5");
    test_option("??IBase@@QEAAHH@Z", "Base::operator&");
    test_option("?ret_fnptrarray@@YAP6AXQAH@ZXZ", "ret_fnptrarray");
    test_option("??Pklass@@QEAAHH@Z", "klass::operator>=");
    test_option("??$forward@P8?$DecoderStream@$01@media@@AEXXZ@std@@YA$$QAP8?$DecoderStream@$01@media@@AEXXZAAP812@AEXXZ@Z", "std::forward<void (__thiscall media::DecoderStream<2>::*)(void)>");
    test_option("?foo_piad@@YAXPIAD@Z", "foo_piad");
    test_option("??4Base@@QEAAHH@Z", "Base::operator=");
    test_option("?s0@PR13182@@3PADA", "PR13182::s0");
    test_option("?foo_papcd@@YAXPEAPECD@Z", "foo_papcd");
    test_option("?x@@YAXMHZZ", "x");
    test_option(
        "??0?$IntTemplate@$0CAB@@@QAE@XZ",
        "IntTemplate<513>::IntTemplate<513>",
    );
    test_option("??0?$ClassTemplate@$J??_9MostGeneral@@$BA@AEA@M@3@@QAE@XZ", "ClassTemplate<{MostGeneral::`vcall'{0}, 0, 12, 4}>::ClassTemplate<{MostGeneral::`vcall'{0}, 0, 12, 4}>");
    test_option("??CBase@@QEAAHXZ", "Base::operator->");
    test_option("?h3@@3QIAHIA", "h3");
    test_option("??OBase@@QEAAHH@Z", "Base::operator>");
    test_option("?M@?1???$L@H@@YAHXZ@4HA", "`L<int>'::`2'::M");
    test_option("??Vklass@@QEAAHH@Z", "klass::operator&&");
    test_option("?c10@@YAACVA@@XZ", "c10");
    test_option("?n@@3U?$J@UN@@$HA@@@A", "n");
    test_option(
        "??0?$Class@$$BY04H@@QEAA@XZ",
        "Class<int[5]>::Class<int[5]>",
    );
    test_option("??2@YAPAXI@Z", "operator new");
    test_option("??_9Base@@$B7AA", "Base::`vcall'{8}");
    test_option("?M@?0??L@@YAHXZ@4HA", "`L'::`1'::M");
    test_option("?foo@A@PR19361@@QIHAEXXZ", "PR19361::A::foo");
    test_option(
        "?k@FTypeWithQuals@@3U?$S@$$A8@@GAAHXZ@1@A",
        "FTypeWithQuals::k",
    );
    test_option(
        "??$FunctionPointerTemplate@$1?spam@@YAXXZ@@YAXXZ",
        "FunctionPointerTemplate<&spam>",
    );
    test_option("?foo_pcrcd@@YAXPECRECD@Z", "foo_pcrcd");
    test_option(
        "?l@FTypeWithQuals@@3U?$S@$$A8@@HAAHXZ@1@A",
        "FTypeWithQuals::l",
    );
    test_option("?mangle_yes_backref1@@YAXQEAH0@Z", "mangle_yes_backref1");
    test_option("?x@@3QEBHEB", "x");
    test_option("?priv_stat_foo@S@@CAXXZ", "S::priv_stat_foo");
    test_option(
        "??$CallMethod@UO@@$H??_91@$BA@AE3@@YAXAAUO@@@Z",
        "CallMethod<O, {O::`vcall'{0}, 4}>",
    );
    test_option(
        "??0?$IntTemplate@$0L@@@QEAA@XZ",
        "IntTemplate<11>::IntTemplate<11>",
    );
    test_option(
        "?bar@NB@PR13207@@YAXV?$Y@VX@NB@PR13207@@@NA@2@@Z",
        "PR13207::NB::bar",
    );
    test_option("?mangle_yes_backref0@@YAXQEAH0@Z", "mangle_yes_backref0");
    test_option("?foo_fnptrconst@@YAXP6AXQAH@Z@Z", "foo_fnptrconst");
    test_option("??_3Base@@QEAAHH@Z", "Base::operator<<=");
    test_option(
        "??_F?$SomeTemplate@H@@QAEXXZ",
        "SomeTemplate<int>::`default constructor closure'",
    );
    test_option("?foo_qapad@@YAXQEAPEAD@Z", "foo_qapad");
    test_option("??_R3Base@@8", "Base::`RTTI Class Hierarchy Descriptor'");
    test_option("?zeta@@YAXP6AHHH@Z@Z", "zeta");
    test_option("?g2@@YAXUS@@0@Z", "g2");
    test_option("?x@@3P6AHMNH@ZEA", "x");
    test_option("?foo_fnptrbackref1@@YAXP6AXQEAH@Z1@Z", "foo_fnptrbackref1");
    test_option("?s1@PR13182@@3PADA", "PR13182::s1");
    test_option("?foo_aay144cbh@@YAXAAY144$$CBH@Z", "foo_aay144cbh");
    test_option("??Dklass@@QEAAHXZ", "klass::operator*");
    test_option(
        "??0?$IntTemplate@$0EAC@@@QEAA@XZ",
        "IntTemplate<1026>::IntTemplate<1026>",
    );
    test_option(
        "??0?$IntTemplate@$0A@@@QEAA@XZ",
        "IntTemplate<0>::IntTemplate<0>",
    );
    test_option(
        "??0?$IntTemplate@$0?8@@QEAA@XZ",
        "IntTemplate<-9>::IntTemplate<-9>",
    );
    test_option(
        "??0?$Class@$$BY04$$CBH@@QEAA@XZ",
        "Class<int const[5]>::Class<int const[5]>",
    );
    test_option("?function_pointer@@YAXV?$C@P6AXXZ@@@Z", "function_pointer");
    test_option(
        "??$unaligned_x@PFAH@@3PFAHA",
        "unaligned_x<int __unaligned *>",
    );
    test_option(".?AVtype_info@@", "type_info");
}

#[test]
fn test_unicode() {
    test(".?AUМосква@@", "struct Москва `RTTI Type Descriptor Name'");
    test(".?AU東京@@", "struct 東京 `RTTI Type Descriptor Name'");
}
