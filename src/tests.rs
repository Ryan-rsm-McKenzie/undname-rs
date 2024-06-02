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
use bstr::ByteSlice as _;
use memchr::memmem;

fn do_test(mangled_name: &[u8], demangled_name: &[u8], partial_match: bool, flags: Flags) {
    let result = crate::demangle(mangled_name.into(), flags);
    match result {
        Ok(haystack) => {
            let matched = if partial_match {
                // this is how llvm checks their tests
                memmem::find(&haystack, demangled_name).is_some()
            } else {
                haystack == demangled_name
            };
            if !matched {
                assert!(
                    false,
                    "'{}' <-- mangled string\n{flags:?} <-- flags\n'{}' <-- expected\n'{}' <-- actual",
                    mangled_name.to_str_lossy(),
                    demangled_name.to_str_lossy(),
                    haystack.to_str_lossy(),
                );
            }
        }
        Err(err) => assert!(
            false,
            "'{}' <-- mangled string\n{err:?} <-- error",
            mangled_name.to_str_lossy()
        ),
    }
}

fn test(mangled_name: &[u8], demangled_name: &[u8]) {
    do_test(mangled_name, demangled_name, true, Flags::default())
}

#[test]
fn test_arg_qualifiers() {
    test(b"?foo@@YAXI@Z", b"void __cdecl foo(unsigned int)");
    test(b"?foo@@YAXN@Z  ", b"void __cdecl foo(double)");
    test(b"?foo_pad@@YAXPAD@Z", b"void __cdecl foo_pad(char *)");
    test(b"?foo_pad@@YAXPEAD@Z", b"void __cdecl foo_pad(char *)");
    test(b"?foo_pbd@@YAXPBD@Z", b"void __cdecl foo_pbd(char const *)");
    test(
        b"?foo_pbd@@YAXPEBD@Z",
        b"void __cdecl foo_pbd(char const *)",
    );
    test(
        b"?foo_pcd@@YAXPCD@Z",
        b"void __cdecl foo_pcd(char volatile *)",
    );
    test(
        b"?foo_pcd@@YAXPECD@Z",
        b"void __cdecl foo_pcd(char volatile *)",
    );
    test(b"?foo_qad@@YAXQAD@Z", b"void __cdecl foo_qad(char *const)");
    test(b"?foo_qad@@YAXQEAD@Z", b"void __cdecl foo_qad(char *const)");
    test(
        b"?foo_rad@@YAXRAD@Z",
        b"void __cdecl foo_rad(char *volatile)",
    );
    test(
        b"?foo_rad@@YAXREAD@Z",
        b"void __cdecl foo_rad(char *volatile)",
    );
    test(
        b"?foo_sad@@YAXSAD@Z",
        b"void __cdecl foo_sad(char *const volatile)",
    );
    test(
        b"?foo_sad@@YAXSEAD@Z",
        b"void __cdecl foo_sad(char *const volatile)",
    );
    test(
        b"?foo_piad@@YAXPIAD@Z",
        b"void __cdecl foo_piad(char *__restrict)",
    );
    test(
        b"?foo_piad@@YAXPEIAD@Z",
        b"void __cdecl foo_piad(char *__restrict)",
    );
    test(
        b"?foo_qiad@@YAXQIAD@Z",
        b"void __cdecl foo_qiad(char *const __restrict)",
    );
    test(
        b"?foo_qiad@@YAXQEIAD@Z",
        b"void __cdecl foo_qiad(char *const __restrict)",
    );
    test(
        b"?foo_riad@@YAXRIAD@Z",
        b"void __cdecl foo_riad(char *volatile __restrict)",
    );
    test(
        b"?foo_riad@@YAXREIAD@Z",
        b"void __cdecl foo_riad(char *volatile __restrict)",
    );
    test(
        b"?foo_siad@@YAXSIAD@Z",
        b"void __cdecl foo_siad(char *const volatile __restrict)",
    );
    test(
        b"?foo_siad@@YAXSEIAD@Z",
        b"void __cdecl foo_siad(char *const volatile __restrict)",
    );
    test(
        b"?foo_papad@@YAXPAPAD@Z",
        b"void __cdecl foo_papad(char **)",
    );
    test(
        b"?foo_papad@@YAXPEAPEAD@Z",
        b"void __cdecl foo_papad(char **)",
    );
    test(
        b"?foo_papbd@@YAXPAPBD@Z",
        b"void __cdecl foo_papbd(char const **)",
    );
    test(
        b"?foo_papbd@@YAXPEAPEBD@Z",
        b"void __cdecl foo_papbd(char const **)",
    );
    test(
        b"?foo_papcd@@YAXPAPCD@Z",
        b"void __cdecl foo_papcd(char volatile **)",
    );
    test(
        b"?foo_papcd@@YAXPEAPECD@Z",
        b"void __cdecl foo_papcd(char volatile **)",
    );
    test(
        b"?foo_pbqad@@YAXPBQAD@Z",
        b"void __cdecl foo_pbqad(char *const *)",
    );
    test(
        b"?foo_pbqad@@YAXPEBQEAD@Z",
        b"void __cdecl foo_pbqad(char *const *)",
    );
    test(
        b"?foo_pcrad@@YAXPCRAD@Z",
        b"void __cdecl foo_pcrad(char *volatile *)",
    );
    test(
        b"?foo_pcrad@@YAXPECREAD@Z",
        b"void __cdecl foo_pcrad(char *volatile *)",
    );
    test(
        b"?foo_qapad@@YAXQAPAD@Z",
        b"void __cdecl foo_qapad(char **const)",
    );
    test(
        b"?foo_qapad@@YAXQEAPEAD@Z",
        b"void __cdecl foo_qapad(char **const)",
    );
    test(
        b"?foo_rapad@@YAXRAPAD@Z",
        b"void __cdecl foo_rapad(char **volatile)",
    );
    test(
        b"?foo_rapad@@YAXREAPEAD@Z",
        b"void __cdecl foo_rapad(char **volatile)",
    );
    test(
        b"?foo_pbqbd@@YAXPBQBD@Z",
        b"void __cdecl foo_pbqbd(char const *const *)",
    );
    test(
        b"?foo_pbqbd@@YAXPEBQEBD@Z",
        b"void __cdecl foo_pbqbd(char const *const *)",
    );
    test(
        b"?foo_pbqcd@@YAXPBQCD@Z",
        b"void __cdecl foo_pbqcd(char volatile *const *)",
    );
    test(
        b"?foo_pbqcd@@YAXPEBQECD@Z",
        b"void __cdecl foo_pbqcd(char volatile *const *)",
    );
    test(
        b"?foo_pcrbd@@YAXPCRBD@Z",
        b"void __cdecl foo_pcrbd(char const *volatile *)",
    );
    test(
        b"?foo_pcrbd@@YAXPECREBD@Z",
        b"void __cdecl foo_pcrbd(char const *volatile *)",
    );
    test(
        b"?foo_pcrcd@@YAXPCRCD@Z",
        b"void __cdecl foo_pcrcd(char volatile *volatile *)",
    );
    test(
        b"?foo_pcrcd@@YAXPECRECD@Z",
        b"void __cdecl foo_pcrcd(char volatile *volatile *)",
    );
    test(b"?foo_aad@@YAXAAD@Z", b"void __cdecl foo_aad(char &)");
    test(b"?foo_aad@@YAXAEAD@Z", b"void __cdecl foo_aad(char &)");
    test(b"?foo_abd@@YAXABD@Z", b"void __cdecl foo_abd(char const &)");
    test(
        b"?foo_abd@@YAXAEBD@Z",
        b"void __cdecl foo_abd(char const &)",
    );
    test(
        b"?foo_aapad@@YAXAAPAD@Z",
        b"void __cdecl foo_aapad(char *&)",
    );
    test(
        b"?foo_aapad@@YAXAEAPEAD@Z",
        b"void __cdecl foo_aapad(char *&)",
    );
    test(
        b"?foo_aapbd@@YAXAAPBD@Z",
        b"void __cdecl foo_aapbd(char const *&)",
    );
    test(
        b"?foo_aapbd@@YAXAEAPEBD@Z",
        b"void __cdecl foo_aapbd(char const *&)",
    );
    test(
        b"?foo_abqad@@YAXABQAD@Z",
        b"void __cdecl foo_abqad(char *const &)",
    );
    test(
        b"?foo_abqad@@YAXAEBQEAD@Z",
        b"void __cdecl foo_abqad(char *const &)",
    );
    test(
        b"?foo_abqbd@@YAXABQBD@Z",
        b"void __cdecl foo_abqbd(char const *const &)",
    );
    test(
        b"?foo_abqbd@@YAXAEBQEBD@Z",
        b"void __cdecl foo_abqbd(char const *const &)",
    );
    test(
        b"?foo_aay144h@@YAXAAY144H@Z",
        b"void __cdecl foo_aay144h(int (&)[5][5])",
    );
    test(
        b"?foo_aay144h@@YAXAEAY144H@Z",
        b"void __cdecl foo_aay144h(int (&)[5][5])",
    );
    test(
        b"?foo_aay144cbh@@YAXAAY144$$CBH@Z",
        b"void __cdecl foo_aay144cbh(int const (&)[5][5])",
    );
    test(
        b"?foo_aay144cbh@@YAXAEAY144$$CBH@Z",
        b"void __cdecl foo_aay144cbh(int const (&)[5][5])",
    );
    test(
        b"?foo_qay144h@@YAX$$QAY144H@Z",
        b"void __cdecl foo_qay144h(int (&&)[5][5])",
    );
    test(
        b"?foo_qay144h@@YAX$$QEAY144H@Z",
        b"void __cdecl foo_qay144h(int (&&)[5][5])",
    );
    test(
        b"?foo_qay144cbh@@YAX$$QAY144$$CBH@Z",
        b"void __cdecl foo_qay144cbh(int const (&&)[5][5])",
    );
    test(
        b"?foo_qay144cbh@@YAX$$QEAY144$$CBH@Z",
        b"void __cdecl foo_qay144cbh(int const (&&)[5][5])",
    );
    test(
        b"?foo_p6ahxz@@YAXP6AHXZ@Z",
        b"void __cdecl foo_p6ahxz(int (__cdecl *)(void))",
    );
    test(
        b"?foo_p6ahxz@@YAXP6AHXZ@Z",
        b"void __cdecl foo_p6ahxz(int (__cdecl *)(void))",
    );
    test(
        b"?foo_a6ahxz@@YAXA6AHXZ@Z",
        b"void __cdecl foo_a6ahxz(int (__cdecl &)(void))",
    );
    test(
        b"?foo_a6ahxz@@YAXA6AHXZ@Z",
        b"void __cdecl foo_a6ahxz(int (__cdecl &)(void))",
    );
    test(
        b"?foo_q6ahxz@@YAX$$Q6AHXZ@Z",
        b"void __cdecl foo_q6ahxz(int (__cdecl &&)(void))",
    );
    test(
        b"?foo_q6ahxz@@YAX$$Q6AHXZ@Z",
        b"void __cdecl foo_q6ahxz(int (__cdecl &&)(void))",
    );
    test(
        b"?foo_qay04h@@YAXQAY04H@Z",
        b"void __cdecl foo_qay04h(int (*const)[5])",
    );
    test(
        b"?foo_qay04h@@YAXQEAY04H@Z",
        b"void __cdecl foo_qay04h(int (*const)[5])",
    );
    test(
        b"?foo_qay04cbh@@YAXQAY04$$CBH@Z",
        b"void __cdecl foo_qay04cbh(int const (*const)[5])",
    );
    test(
        b"?foo_qay04cbh@@YAXQEAY04$$CBH@Z",
        b"void __cdecl foo_qay04cbh(int const (*const)[5])",
    );
    test(b"?foo@@YAXPAY02N@Z", b"void __cdecl foo(double (*)[3])");
    test(b"?foo@@YAXPEAY02N@Z", b"void __cdecl foo(double (*)[3])");
    test(b"?foo@@YAXQAN@Z", b"void __cdecl foo(double *const)");
    test(b"?foo@@YAXQEAN@Z", b"void __cdecl foo(double *const)");
    test(
        b"?foo_const@@YAXQBN@Z",
        b"void __cdecl foo_const(double const *const)",
    );
    test(
        b"?foo_const@@YAXQEBN@Z",
        b"void __cdecl foo_const(double const *const)",
    );
    test(
        b"?foo_volatile@@YAXQCN@Z",
        b"void __cdecl foo_volatile(double volatile *const)",
    );
    test(
        b"?foo_volatile@@YAXQECN@Z",
        b"void __cdecl foo_volatile(double volatile *const)",
    );
    test(
        b"?foo@@YAXPAY02NQBNN@Z",
        b"void __cdecl foo(double (*)[3], double const *const, double)",
    );
    test(
        b"?foo@@YAXPEAY02NQEBNN@Z",
        b"void __cdecl foo(double (*)[3], double const *const, double)",
    );
    test(
        b"?foo_fnptrconst@@YAXP6AXQAH@Z@Z",
        b"void __cdecl foo_fnptrconst(void (__cdecl *)(int *const))",
    );
    test(
        b"?foo_fnptrconst@@YAXP6AXQEAH@Z@Z",
        b"void __cdecl foo_fnptrconst(void (__cdecl *)(int *const))",
    );
    test(
        b"?foo_fnptrarray@@YAXP6AXQAH@Z@Z",
        b"void __cdecl foo_fnptrarray(void (__cdecl *)(int *const))",
    );
    test(
        b"?foo_fnptrarray@@YAXP6AXQEAH@Z@Z",
        b"void __cdecl foo_fnptrarray(void (__cdecl *)(int *const))",
    );
    test(b"?foo_fnptrbackref1@@YAXP6AXQAH@Z1@Z", b"void __cdecl foo_fnptrbackref1(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test(b"?foo_fnptrbackref1@@YAXP6AXQEAH@Z1@Z", b"void __cdecl foo_fnptrbackref1(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test(b"?foo_fnptrbackref2@@YAXP6AXQAH@Z1@Z", b"void __cdecl foo_fnptrbackref2(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test(b"?foo_fnptrbackref2@@YAXP6AXQEAH@Z1@Z", b"void __cdecl foo_fnptrbackref2(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test(b"?foo_fnptrbackref3@@YAXP6AXQAH@Z1@Z", b"void __cdecl foo_fnptrbackref3(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test(b"?foo_fnptrbackref3@@YAXP6AXQEAH@Z1@Z", b"void __cdecl foo_fnptrbackref3(void (__cdecl *)(int *const), void (__cdecl *)(int *const))");
    test(
        b"?foo_fnptrbackref4@@YAXP6AXPAH@Z1@Z",
        b"void __cdecl foo_fnptrbackref4(void (__cdecl *)(int *), void (__cdecl *)(int *))",
    );
    test(
        b"?foo_fnptrbackref4@@YAXP6AXPEAH@Z1@Z",
        b"void __cdecl foo_fnptrbackref4(void (__cdecl *)(int *), void (__cdecl *)(int *))",
    );
    test(
        b"?ret_fnptrarray@@YAP6AXQAH@ZXZ",
        b"void (__cdecl * __cdecl ret_fnptrarray(void))(int *const)",
    );
    test(
        b"?ret_fnptrarray@@YAP6AXQEAH@ZXZ",
        b"void (__cdecl * __cdecl ret_fnptrarray(void))(int *const)",
    );
    test(
        b"?mangle_no_backref0@@YAXQAHPAH@Z",
        b"void __cdecl mangle_no_backref0(int *const, int *)",
    );
    test(
        b"?mangle_no_backref0@@YAXQEAHPEAH@Z",
        b"void __cdecl mangle_no_backref0(int *const, int *)",
    );
    test(
        b"?mangle_no_backref1@@YAXQAHQAH@Z",
        b"void __cdecl mangle_no_backref1(int *const, int *const)",
    );
    test(
        b"?mangle_no_backref1@@YAXQEAHQEAH@Z",
        b"void __cdecl mangle_no_backref1(int *const, int *const)",
    );
    test(
        b"?mangle_no_backref2@@YAXP6AXXZP6AXXZ@Z",
        b"void __cdecl mangle_no_backref2(void (__cdecl *)(void), void (__cdecl *)(void))",
    );
    test(
        b"?mangle_no_backref2@@YAXP6AXXZP6AXXZ@Z",
        b"void __cdecl mangle_no_backref2(void (__cdecl *)(void), void (__cdecl *)(void))",
    );
    test(
        b"?mangle_yes_backref0@@YAXQAH0@Z",
        b"void __cdecl mangle_yes_backref0(int *const, int *const)",
    );
    test(
        b"?mangle_yes_backref0@@YAXQEAH0@Z",
        b"void __cdecl mangle_yes_backref0(int *const, int *const)",
    );
    test(
        b"?mangle_yes_backref1@@YAXQAH0@Z",
        b"void __cdecl mangle_yes_backref1(int *const, int *const)",
    );
    test(
        b"?mangle_yes_backref1@@YAXQEAH0@Z",
        b"void __cdecl mangle_yes_backref1(int *const, int *const)",
    );
    test(b"?mangle_yes_backref2@@YAXQBQ6AXXZ0@Z", b"void __cdecl mangle_yes_backref2(void (__cdecl *const *const)(void), void (__cdecl *const *const)(void))");
    test(b"?mangle_yes_backref2@@YAXQEBQ6AXXZ0@Z", b"void __cdecl mangle_yes_backref2(void (__cdecl *const *const)(void), void (__cdecl *const *const)(void))");
    test(b"?mangle_yes_backref3@@YAXQAP6AXXZ0@Z", b"void __cdecl mangle_yes_backref3(void (__cdecl **const)(void), void (__cdecl **const)(void))");
    test(b"?mangle_yes_backref3@@YAXQEAP6AXXZ0@Z", b"void __cdecl mangle_yes_backref3(void (__cdecl **const)(void), void (__cdecl **const)(void))");
    test(
        b"?mangle_yes_backref4@@YAXQIAH0@Z",
        b"void __cdecl mangle_yes_backref4(int *const __restrict, int *const __restrict)",
    );
    test(
        b"?mangle_yes_backref4@@YAXQEIAH0@Z",
        b"void __cdecl mangle_yes_backref4(int *const __restrict, int *const __restrict)",
    );
    test(
        b"?pr23325@@YAXQBUS@@0@Z",
        b"void __cdecl pr23325(struct S const *const, struct S const *const)",
    );
    test(
        b"?pr23325@@YAXQEBUS@@0@Z",
        b"void __cdecl pr23325(struct S const *const, struct S const *const)",
    );
}

#[test]
fn test_back_references() {
    test(
        b"?f1@@YAXPBD0@Z",
        b"void __cdecl f1(char const *, char const *)",
    );
    test(
        b"?f2@@YAXPBDPAD@Z",
        b"void __cdecl f2(char const *, char *)",
    );
    test(
        b"?f3@@YAXHPBD0@Z",
        b"void __cdecl f3(int, char const *, char const *)",
    );
    test(
        b"?f4@@YAPBDPBD0@Z",
        b"char const * __cdecl f4(char const *, char const *)",
    );
    test(b"?f5@@YAXPBDIDPBX0I@Z", b"void __cdecl f5(char const *, unsigned int, char, void const *, char const *, unsigned int)");
    test(b"?f6@@YAX_N0@Z", b"void __cdecl f6(bool, bool)");
    test(
        b"?f7@@YAXHPAHH0_N1PA_N@Z",
        b"void __cdecl f7(int, int *, int, int *, bool, bool, bool *)",
    );
    test(b"?g1@@YAXUS@@@Z", b"void __cdecl g1(struct S)");
    test(b"?g2@@YAXUS@@0@Z", b"void __cdecl g2(struct S, struct S)");
    test(
        b"?g3@@YAXUS@@0PAU1@1@Z",
        b"void __cdecl g3(struct S, struct S, struct S *, struct S *)",
    );
    test(
        b"?g4@@YAXPBDPAUS@@01@Z",
        b"void __cdecl g4(char const *, struct S *, char const *, struct S *)",
    );
    test(b"?mbb@S@@QAEX_N0@Z", b"void __thiscall S::mbb(bool, bool)");
    test(b"?h1@@YAXPBD0P6AXXZ1@Z", b"void __cdecl h1(char const *, char const *, void (__cdecl *)(void), void (__cdecl *)(void))");
    test(
        b"?h2@@YAXP6AXPAX@Z0@Z",
        b"void __cdecl h2(void (__cdecl *)(void *), void *)",
    );
    test(b"?h3@@YAP6APAHPAH0@ZP6APAH00@Z10@Z", b"int * (__cdecl * __cdecl h3(int * (__cdecl *)(int *, int *), int * (__cdecl *)(int *, int *), int *))(int *, int *)");
    test(b"?foo@0@YAXXZ", b"void __cdecl foo::foo(void)");
    test(
        b"??$?HH@S@@QEAAAEAU0@H@Z",
        b"struct S & __cdecl S::operator+<int>(int)",
    );
    test(
        b"?foo_abbb@@YAXV?$A@V?$B@D@@V1@V1@@@@Z",
        b"void __cdecl foo_abbb(class A<class B<char>, class B<char>, class B<char>>)",
    );
    test(
        b"?foo_abb@@YAXV?$A@DV?$B@D@@V1@@@@Z",
        b"void __cdecl foo_abb(class A<char, class B<char>, class B<char>>)",
    );
    test(
        b"?foo_abc@@YAXV?$A@DV?$B@D@@V?$C@D@@@@@Z",
        b"void __cdecl foo_abc(class A<char, class B<char>, class C<char>>)",
    );
    test(
        b"?foo_bt@@YAX_NV?$B@$$A6A_N_N@Z@@@Z",
        b"void __cdecl foo_bt(bool, class B<bool __cdecl(bool)>)",
    );
    test(
        b"?foo_abbb@@YAXV?$A@V?$B@D@N@@V12@V12@@N@@@Z",
        b"void __cdecl foo_abbb(class N::A<class N::B<char>, class N::B<char>, class N::B<char>>)",
    );
    test(
        b"?foo_abb@@YAXV?$A@DV?$B@D@N@@V12@@N@@@Z",
        b"void __cdecl foo_abb(class N::A<char, class N::B<char>, class N::B<char>>)",
    );
    test(
        b"?foo_abc@@YAXV?$A@DV?$B@D@N@@V?$C@D@2@@N@@@Z",
        b"void __cdecl foo_abc(class N::A<char, class N::B<char>, class N::C<char>>)",
    );
    test(
        b"?abc_foo@@YA?AV?$A@DV?$B@D@N@@V?$C@D@2@@N@@XZ",
        b"class N::A<char, class N::B<char>, class N::C<char>> __cdecl abc_foo(void)",
    );
    test(
        b"?z_foo@@YA?AVZ@N@@V12@@Z",
        b"class N::Z __cdecl z_foo(class N::Z)",
    );
    test(
        b"?b_foo@@YA?AV?$B@D@N@@V12@@Z",
        b"class N::B<char> __cdecl b_foo(class N::B<char>)",
    );
    test(
        b"?d_foo@@YA?AV?$D@DD@N@@V12@@Z",
        b"class N::D<char, char> __cdecl d_foo(class N::D<char, char>)",
    );
    test(b"?abc_foo_abc@@YA?AV?$A@DV?$B@D@N@@V?$C@D@2@@N@@V12@@Z", b"class N::A<char, class N::B<char>, class N::C<char>> __cdecl abc_foo_abc(class N::A<char, class N::B<char>, class N::C<char>>)");
    test(
        b"?foo5@@YAXV?$Y@V?$Y@V?$Y@V?$Y@VX@NA@@@NB@@@NA@@@NB@@@NA@@@Z",
        b"void __cdecl foo5(class NA::Y<class NB::Y<class NA::Y<class NB::Y<class NA::X>>>>)",
    );
    test(
        b"?foo11@@YAXV?$Y@VX@NA@@@NA@@V1NB@@@Z",
        b"void __cdecl foo11(class NA::Y<class NA::X>, class NB::Y<class NA::X>)",
    );
    test(
        b"?foo112@@YAXV?$Y@VX@NA@@@NA@@V?$Y@VX@NB@@@NB@@@Z",
        b"void __cdecl foo112(class NA::Y<class NA::X>, class NB::Y<class NB::X>)",
    );
    test(b"?foo22@@YAXV?$Y@V?$Y@VX@NA@@@NB@@@NA@@V?$Y@V?$Y@VX@NA@@@NA@@@NB@@@Z", b"void __cdecl foo22(class NA::Y<class NB::Y<class NA::X>>, class NB::Y<class NA::Y<class NA::X>>)");
    test(
        b"?foo@L@PR13207@@QAEXV?$I@VA@PR13207@@@2@@Z",
        b"void __thiscall PR13207::L::foo(class PR13207::I<class PR13207::A>)",
    );
    test(
        b"?foo@PR13207@@YAXV?$I@VA@PR13207@@@1@@Z",
        b"void __cdecl PR13207::foo(class PR13207::I<class PR13207::A>)",
    );
    test(b"?foo2@PR13207@@YAXV?$I@VA@PR13207@@@1@0@Z", b"void __cdecl PR13207::foo2(class PR13207::I<class PR13207::A>, class PR13207::I<class PR13207::A>)");
    test(
        b"?bar@PR13207@@YAXV?$J@VA@PR13207@@VB@2@@1@@Z",
        b"void __cdecl PR13207::bar(class PR13207::J<class PR13207::A, class PR13207::B>)",
    );
    test(b"?spam@PR13207@@YAXV?$K@VA@PR13207@@VB@2@VC@2@@1@@Z", b"void __cdecl PR13207::spam(class PR13207::K<class PR13207::A, class PR13207::B, class PR13207::C>)");
    test(b"?baz@PR13207@@YAXV?$K@DV?$F@D@PR13207@@V?$I@D@2@@1@@Z", b"void __cdecl PR13207::baz(class PR13207::K<char, class PR13207::F<char>, class PR13207::I<char>>)");
    test(b"?qux@PR13207@@YAXV?$K@DV?$I@D@PR13207@@V12@@1@@Z", b"void __cdecl PR13207::qux(class PR13207::K<char, class PR13207::I<char>, class PR13207::I<char>>)");
    test(
        b"?foo@NA@PR13207@@YAXV?$Y@VX@NA@PR13207@@@12@@Z",
        b"void __cdecl PR13207::NA::foo(class PR13207::NA::Y<class PR13207::NA::X>)",
    );
    test(b"?foofoo@NA@PR13207@@YAXV?$Y@V?$Y@VX@NA@PR13207@@@NA@PR13207@@@12@@Z", b"void __cdecl PR13207::NA::foofoo(class PR13207::NA::Y<class PR13207::NA::Y<class PR13207::NA::X>>)");
    test(
        b"?foo@NB@PR13207@@YAXV?$Y@VX@NA@PR13207@@@12@@Z",
        b"void __cdecl PR13207::NB::foo(class PR13207::NB::Y<class PR13207::NA::X>)",
    );
    test(
        b"?bar@NB@PR13207@@YAXV?$Y@VX@NB@PR13207@@@NA@2@@Z",
        b"void __cdecl PR13207::NB::bar(class PR13207::NA::Y<class PR13207::NB::X>)",
    );
    test(
        b"?spam@NB@PR13207@@YAXV?$Y@VX@NA@PR13207@@@NA@2@@Z",
        b"void __cdecl PR13207::NB::spam(class PR13207::NA::Y<class PR13207::NA::X>)",
    );
    test(b"?foobar@NB@PR13207@@YAXV?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NA@2@V312@@Z", b"void __cdecl PR13207::NB::foobar(class PR13207::NA::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>)");
    test(b"?foobarspam@NB@PR13207@@YAXV?$Y@VX@NB@PR13207@@@12@V?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NA@2@V412@@Z", b"void __cdecl PR13207::NB::foobarspam(class PR13207::NB::Y<class PR13207::NB::X>, class PR13207::NA::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>)");
    test(b"?foobarbaz@NB@PR13207@@YAXV?$Y@VX@NB@PR13207@@@12@V?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NA@2@V412@2@Z", b"void __cdecl PR13207::NB::foobarbaz(class PR13207::NB::Y<class PR13207::NB::X>, class PR13207::NA::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>)");
    test(b"?foobarbazqux@NB@PR13207@@YAXV?$Y@VX@NB@PR13207@@@12@V?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NA@2@V412@2V?$Y@V?$Y@V?$Y@VX@NB@PR13207@@@NB@PR13207@@@NB@PR13207@@@52@@Z", b"void __cdecl PR13207::NB::foobarbazqux(class PR13207::NB::Y<class PR13207::NB::X>, class PR13207::NA::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>, class PR13207::NA::Y<class PR13207::NB::Y<class PR13207::NB::Y<class PR13207::NB::X>>>)");
    test(
        b"?foo@NC@PR13207@@YAXV?$Y@VX@NB@PR13207@@@12@@Z",
        b"void __cdecl PR13207::NC::foo(class PR13207::NC::Y<class PR13207::NB::X>)",
    );
    test(b"?foobar@NC@PR13207@@YAXV?$Y@V?$Y@V?$Y@VX@NA@PR13207@@@NA@PR13207@@@NB@PR13207@@@12@@Z", b"void __cdecl PR13207::NC::foobar(class PR13207::NC::Y<class PR13207::NB::Y<class PR13207::NA::Y<class PR13207::NA::X>>>)");
    test(
        b"?fun_normal@fn_space@@YA?AURetVal@1@H@Z",
        b"struct fn_space::RetVal __cdecl fn_space::fun_normal(int)",
    );
    test(
        b"??$fun_tmpl@H@fn_space@@YA?AURetVal@0@ABH@Z",
        b"struct fn_space::RetVal __cdecl fn_space::fun_tmpl<int>(int const &)",
    );
    test(b"??$fun_tmpl_recurse@H$1??$fun_tmpl_recurse@H$1?ident@fn_space@@YA?AURetVal@2@H@Z@fn_space@@YA?AURetVal@1@H@Z@fn_space@@YA?AURetVal@0@H@Z", b"struct fn_space::RetVal __cdecl fn_space::fun_tmpl_recurse<int, &struct fn_space::RetVal __cdecl fn_space::fun_tmpl_recurse<int, &struct fn_space::RetVal __cdecl fn_space::ident(int)>(int)>(int)");
    test(b"??$fun_tmpl_recurse@H$1?ident@fn_space@@YA?AURetVal@2@H@Z@fn_space@@YA?AURetVal@0@H@Z", b"struct fn_space::RetVal __cdecl fn_space::fun_tmpl_recurse<int, &struct fn_space::RetVal __cdecl fn_space::ident(int)>(int)");
    test(b"?AddEmitPasses@EmitAssemblyHelper@?A0x43583946@@AEAA_NAEAVPassManager@legacy@llvm@@W4BackendAction@clang@@AEAVraw_pwrite_stream@5@PEAV85@@Z", b"bool __cdecl `anonymous namespace'::EmitAssemblyHelper::AddEmitPasses(class llvm::legacy::PassManager &, enum clang::BackendAction, class llvm::raw_pwrite_stream &, class llvm::raw_pwrite_stream *)");
    test(b"??$forward@P8?$DecoderStream@$01@media@@AEXXZ@std@@YA$$QAP8?$DecoderStream@$01@media@@AEXXZAAP812@AEXXZ@Z", b"void (__thiscall media::DecoderStream<2>::*&& __cdecl std::forward<void (__thiscall media::DecoderStream<2>::*)(void)>(void (__thiscall media::DecoderStream<2>::*&)(void)))(void)");
}

#[test]
fn test_basic() {
    test(b"?x@@3HA", b"int x");
    test(b"?x@@3PEAHEA", b"int *x");
    test(b"?x@@3PEAPEAHEA", b"int **x");
    test(b"?foo@@3Y123KA", b"unsigned long foo[3][4]");
    test(b"?x@@3PEAY02HEA", b"int (*x)[3]");
    test(b"?x@@3PEAY124HEA", b"int (*x)[3][5]");
    test(b"?x@@3PEAY02$$CBHEA", b"int const (*x)[3]");
    test(b"?x@@3PEAEEA", b"unsigned char *x");
    test(b"?y@@3PEAGEA", b"unsigned short *y");
    test(b"?z@@3PEAKEA", b"unsigned long *z");
    test(b"?x@@3PEAY1NKM@5HEA", b"int (*x)[3500][6]");
    test(b"?x@@YAXMH@Z", b"void __cdecl x(float, int)");
    test(b"?x@@YAXMHZZ", b"void __cdecl x(float, int, ...)");
    test(b"?x@@YAXZZ", b"void __cdecl x(...)");
    test(b"?x@@3P6AHMNH@ZEA", b"int (__cdecl *x)(float, double, int)");
    test(
        b"?x@@3P6AHP6AHM@ZN@ZEA",
        b"int (__cdecl *x)(int (__cdecl *)(float), double)",
    );
    test(
        b"?x@@3P6AHP6AHM@Z0@ZEA",
        b"int (__cdecl *x)(int (__cdecl *)(float), int (__cdecl *)(float))",
    );
    test(b"?x@ns@@3HA", b"int ns::x");
    test(b"?x@@3PEAHEA", b"int *x");
    test(b"?x@@3PEBHEB", b"int const *x");
    test(b"?x@@3QEAHEA", b"int *const x");
    test(b"?x@@3QEBHEB", b"int const *const x");
    test(b"?x@@3AEBHEB", b"int const &x");
    test(b"?x@@3PEAUty@@EA", b"struct ty *x");
    test(b"?x@@3PEATty@@EA", b"union ty *x");
    test(b"?x@@3PEAVty@@EA", b"class ty *x");
    test(b"?x@@3PEAW4ty@@EA", b"enum ty *x");
    test(b"?x@@3PEAV?$tmpl@H@@EA", b"class tmpl<int> *x");
    test(b"?x@@3PEAU?$tmpl@H@@EA", b"struct tmpl<int> *x");
    test(b"?x@@3PEAT?$tmpl@H@@EA", b"union tmpl<int> *x");
    test(b"?instance@@3Vklass@@A", b"class klass instance");
    test(
        b"?instance$initializer$@@3P6AXXZEA",
        b"void (__cdecl *instance$initializer$)(void)",
    );
    test(b"??0klass@@QEAA@XZ", b"__cdecl klass::klass(void)");
    test(b"??1klass@@QEAA@XZ", b"__cdecl klass::~klass(void)");
    test(
        b"?x@@YAHPEAVklass@@AEAV1@@Z",
        b"int __cdecl x(class klass *, class klass &)",
    );
    test(
        b"?x@ns@@3PEAV?$klass@HH@1@EA",
        b"class ns::klass<int, int> *ns::x",
    );
    test(
        b"?fn@?$klass@H@ns@@QEBAIXZ",
        b"unsigned int __cdecl ns::klass<int>::fn(void) const",
    );
    test(
        b"??4klass@@QEAAAEBV0@AEBV0@@Z",
        b"class klass const & __cdecl klass::operator=(class klass const &)",
    );
    test(
        b"??7klass@@QEAA_NXZ",
        b"bool __cdecl klass::operator!(void)",
    );
    test(
        b"??8klass@@QEAA_NAEBV0@@Z",
        b"bool __cdecl klass::operator==(class klass const &)",
    );
    test(
        b"??9klass@@QEAA_NAEBV0@@Z",
        b"bool __cdecl klass::operator!=(class klass const &)",
    );
    test(
        b"??Aklass@@QEAAH_K@Z",
        b"int __cdecl klass::operator[](unsigned __int64)",
    );
    test(b"??Cklass@@QEAAHXZ", b"int __cdecl klass::operator->(void)");
    test(b"??Dklass@@QEAAHXZ", b"int __cdecl klass::operator*(void)");
    test(b"??Eklass@@QEAAHXZ", b"int __cdecl klass::operator++(void)");
    test(b"??Eklass@@QEAAHH@Z", b"int __cdecl klass::operator++(int)");
    test(b"??Fklass@@QEAAHXZ", b"int __cdecl klass::operator--(void)");
    test(b"??Fklass@@QEAAHH@Z", b"int __cdecl klass::operator--(int)");
    test(b"??Hklass@@QEAAHH@Z", b"int __cdecl klass::operator+(int)");
    test(b"??Gklass@@QEAAHH@Z", b"int __cdecl klass::operator-(int)");
    test(b"??Iklass@@QEAAHH@Z", b"int __cdecl klass::operator&(int)");
    test(
        b"??Jklass@@QEAAHH@Z",
        b"int __cdecl klass::operator->*(int)",
    );
    test(b"??Kklass@@QEAAHH@Z", b"int __cdecl klass::operator/(int)");
    test(b"??Mklass@@QEAAHH@Z", b"int __cdecl klass::operator<(int)");
    test(b"??Nklass@@QEAAHH@Z", b"int __cdecl klass::operator<=(int)");
    test(b"??Oklass@@QEAAHH@Z", b"int __cdecl klass::operator>(int)");
    test(b"??Pklass@@QEAAHH@Z", b"int __cdecl klass::operator>=(int)");
    test(b"??Qklass@@QEAAHH@Z", b"int __cdecl klass::operator,(int)");
    test(b"??Rklass@@QEAAHH@Z", b"int __cdecl klass::operator()(int)");
    test(b"??Sklass@@QEAAHXZ", b"int __cdecl klass::operator~(void)");
    test(b"??Tklass@@QEAAHH@Z", b"int __cdecl klass::operator^(int)");
    test(b"??Uklass@@QEAAHH@Z", b"int __cdecl klass::operator|(int)");
    test(b"??Vklass@@QEAAHH@Z", b"int __cdecl klass::operator&&(int)");
    test(b"??Wklass@@QEAAHH@Z", b"int __cdecl klass::operator||(int)");
    test(b"??Xklass@@QEAAHH@Z", b"int __cdecl klass::operator*=(int)");
    test(b"??Yklass@@QEAAHH@Z", b"int __cdecl klass::operator+=(int)");
    test(b"??Zklass@@QEAAHH@Z", b"int __cdecl klass::operator-=(int)");
    test(
        b"??_0klass@@QEAAHH@Z",
        b"int __cdecl klass::operator/=(int)",
    );
    test(
        b"??_1klass@@QEAAHH@Z",
        b"int __cdecl klass::operator%=(int)",
    );
    test(
        b"??_2klass@@QEAAHH@Z",
        b"int __cdecl klass::operator>>=(int)",
    );
    test(
        b"??_3klass@@QEAAHH@Z",
        b"int __cdecl klass::operator<<=(int)",
    );
    test(
        b"??_6klass@@QEAAHH@Z",
        b"int __cdecl klass::operator^=(int)",
    );
    test(
        b"??6@YAAEBVklass@@AEBV0@H@Z",
        b"class klass const & __cdecl operator<<(class klass const &, int)",
    );
    test(
        b"??5@YAAEBVklass@@AEBV0@_K@Z",
        b"class klass const & __cdecl operator>>(class klass const &, unsigned __int64)",
    );
    test(
        b"??2@YAPEAX_KAEAVklass@@@Z",
        b"void * __cdecl operator new(unsigned __int64, class klass &)",
    );
    test(
        b"??_U@YAPEAX_KAEAVklass@@@Z",
        b"void * __cdecl operator new[](unsigned __int64, class klass &)",
    );
    test(
        b"??3@YAXPEAXAEAVklass@@@Z",
        b"void __cdecl operator delete(void *, class klass &)",
    );
    test(
        b"??_V@YAXPEAXAEAVklass@@@Z",
        b"void __cdecl operator delete[](void *, class klass &)",
    );
    test(
        b"?A@?A0x43583946@@3VB@@B",
        b"class B const `anonymous namespace'::A",
    );
}

#[test]
fn test_conversion_operators() {
    test(
        b"??$?BH@TemplateOps@@QAEHXZ",
        b"int __thiscall TemplateOps::operator<int> int(void)",
    );
    test(b"??BOps@@QAEHXZ", b"int __thiscall Ops::operator int(void)");
    test(
        b"??BConstOps@@QAE?BHXZ",
        b"int const __thiscall ConstOps::operator int const(void)",
    );
    test(
        b"??BVolatileOps@@QAE?CHXZ",
        b"int volatile __thiscall VolatileOps::operator int volatile(void)",
    );
    test(
        b"??BConstVolatileOps@@QAE?DHXZ",
        b"int const volatile __thiscall ConstVolatileOps::operator int const volatile(void)",
    );
    test(
        b"??$?BN@TemplateOps@@QAENXZ",
        b"double __thiscall TemplateOps::operator<double> double(void)",
    );
    test(
        b"??BOps@@QAENXZ",
        b"double __thiscall Ops::operator double(void)",
    );
    test(
        b"??BConstOps@@QAE?BNXZ",
        b"double const __thiscall ConstOps::operator double const(void)",
    );
    test(
        b"??BVolatileOps@@QAE?CNXZ",
        b"double volatile __thiscall VolatileOps::operator double volatile(void)",
    );
    test(
        b"??BConstVolatileOps@@QAE?DNXZ",
        b"double const volatile __thiscall ConstVolatileOps::operator double const volatile(void)",
    );
    test(
        b"??BCompoundTypeOps@@QAEPAHXZ",
        b"nt * __thiscall CompoundTypeOps::operator int *(void)",
    );
    test(
        b"??BCompoundTypeOps@@QAEPBHXZ",
        b"int const * __thiscall CompoundTypeOps::operator int const *(void)",
    );
    test(
        b"??BCompoundTypeOps@@QAE$$QAHXZ",
        b"int && __thiscall CompoundTypeOps::operator int &&(void)",
    );
    test(
        b"??BCompoundTypeOps@@QAE?AU?$Foo@H@@XZ",
        b"struct Foo<int> __thiscall CompoundTypeOps::operator struct Foo<int>(void)",
    );
    test(b"??$?BH@CompoundTypeOps@@QAE?AU?$Bar@U?$Foo@H@@@@XZ", b"struct Bar<struct Foo<int>> __thiscall CompoundTypeOps::operator<int> struct Bar<struct Foo<int>>(void)");
    test(
        b"??$?BPAH@TemplateOps@@QAEPAHXZ",
        b"int * __thiscall TemplateOps::operator<int *> int *(void)",
    );
}

#[test]
fn test_cxx11() {
    test(
        b"?a@FTypeWithQuals@@3U?$S@$$A8@@BAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) const> FTypeWithQuals::a",
    );
    test(
        b"?b@FTypeWithQuals@@3U?$S@$$A8@@CAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) volatile> FTypeWithQuals::b",
    );
    test(
        b"?c@FTypeWithQuals@@3U?$S@$$A8@@IAAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) __restrict> FTypeWithQuals::c",
    );
    test(
        b"?d@FTypeWithQuals@@3U?$S@$$A8@@GBAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) const &> FTypeWithQuals::d",
    );
    test(
        b"?e@FTypeWithQuals@@3U?$S@$$A8@@GCAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) volatile &> FTypeWithQuals::e",
    );
    test(
        b"?f@FTypeWithQuals@@3U?$S@$$A8@@IGAAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) __restrict &> FTypeWithQuals::f",
    );
    test(
        b"?g@FTypeWithQuals@@3U?$S@$$A8@@HBAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) const &&> FTypeWithQuals::g",
    );
    test(
        b"?h@FTypeWithQuals@@3U?$S@$$A8@@HCAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) volatile &&> FTypeWithQuals::h",
    );
    test(
        b"?i@FTypeWithQuals@@3U?$S@$$A8@@IHAAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) __restrict &&> FTypeWithQuals::i",
    );
    test(
        b"?j@FTypeWithQuals@@3U?$S@$$A6AHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::j",
    );
    test(
        b"?k@FTypeWithQuals@@3U?$S@$$A8@@GAAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) &> FTypeWithQuals::k",
    );
    test(
        b"?l@FTypeWithQuals@@3U?$S@$$A8@@HAAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void) &&> FTypeWithQuals::l",
    );
    test(b"?Char16Var@@3_SA", b"char16_t Char16Var");
    test(b"?Char32Var@@3_UA", b"char32_t Char32Var");
    test(b"?LRef@@YAXAAH@Z", b"void __cdecl LRef(int &)");
    test(b"?RRef@@YAH$$QAH@Z", b"int __cdecl RRef(int &&)");
    test(b"?Null@@YAX$$T@Z", b"void __cdecl Null(std::nullptr_t)");
    test(b"?fun@PR18022@@YA?AU<unnamed-type-a>@1@U21@0@Z", b"struct PR18022::<unnamed-type-a> __cdecl PR18022::fun(struct PR18022::<unnamed-type-a>, struct PR18022::<unnamed-type-a>)");
    test(b"?lambda@?1??define_lambda@@YAHXZ@4V<lambda_1>@?0??1@YAHXZ@A", b"class `int __cdecl define_lambda(void)'::`1'::<lambda_1> `int __cdecl define_lambda(void)'::`2'::lambda");
    test(
        b"??R<lambda_1>@?0??define_lambda@@YAHXZ@QBE@XZ",
        b"__thiscall `int __cdecl define_lambda(void)'::`1'::<lambda_1>::operator()(void) const",
    );
    test(
        b"?local@?2???R<lambda_1>@?0??define_lambda@@YAHXZ@QBE@XZ@4HA",
        b"__thiscall `int __cdecl define_lambda(void)'::`1'::<lambda_1>::operator()(void) const",
    );
    test(b"??$use_lambda_arg@V<lambda_1>@?0??call_with_lambda_arg1@@YAXXZ@@@YAXV<lambda_1>@?0??call_with_lambda_arg1@@YAXXZ@@Z", b"void __cdecl use_lambda_arg<class `void __cdecl call_with_lambda_arg1(void)'::`1'::<lambda_1>>(class `void __cdecl call_with_lambda_arg1(void)'::`1'::<lambda_1>)");
    test(
        b"?foo@A@PR19361@@QIGAEXXZ",
        b"void __thiscall PR19361::A::foo(void) __restrict &",
    );
    test(
        b"?foo@A@PR19361@@QIHAEXXZ",
        b"void __thiscall PR19361::A::foo(void) __restrict &&",
    );
    test(
        b"??__K_deg@@YAHO@Z",
        b"int __cdecl operator \"\"_deg(long double)",
    );
    test(
        b"??$templ_fun_with_pack@$S@@YAXXZ",
        b"void __cdecl templ_fun_with_pack<>(void)",
    );
    test(
        b"??$func@H$$ZH@@YAHAEBU?$Foo@H@@0@Z",
        b"int __cdecl func<int, int>(struct Foo<int> const &, struct Foo<int> const &)",
    );
    test(
        b"??$templ_fun_with_ty_pack@$$$V@@YAXXZ",
        b"void __cdecl templ_fun_with_ty_pack<>(void)",
    );
    test(
        b"??$templ_fun_with_ty_pack@$$V@@YAXXZ",
        b"void __cdecl templ_fun_with_ty_pack<>(void)",
    );
    test(
        b"??$f@$$YAliasA@PR20047@@@PR20047@@YAXXZ",
        b"void __cdecl PR20047::f<PR20047::AliasA>(void)",
    );
    test(
        b"?f@UnnamedType@@YAXAAU<unnamed-type-TD>@A@1@@Z",
        b"void __cdecl UnnamedType::f(struct UnnamedType::A::<unnamed-type-TD> &)",
    );
    test(
        b"?f@UnnamedType@@YAXPAW4<unnamed-type-e>@?$B@H@1@@Z",
        b"void __cdecl UnnamedType::f(enum UnnamedType::B<int>::<unnamed-type-e> *)",
    );
    test(b"??$f@W4<unnamed-type-E>@?1??g@PR24651@@YAXXZ@@PR24651@@YAXW4<unnamed-type-E>@?1??g@0@YAXXZ@@Z", b"void __cdecl PR24651::f<enum `void __cdecl PR24651::g(void)'::`2'::<unnamed-type-E>>(enum `void __cdecl PR24651::g(void)'::`2'::<unnamed-type-E>)");
    test(b"??$f@T<unnamed-type-$S1>@PR18204@@@PR18204@@YAHPAT<unnamed-type-$S1>@0@@Z", b"int __cdecl PR18204::f<union PR18204::<unnamed-type-$S1>>(union PR18204::<unnamed-type-$S1> *)");
    test(
        b"??R<lambda_0>@?0??PR26105@@YAHXZ@QBE@H@Z",
        b"public: __thiscall `int __cdecl PR26105(void)'::`1'::<lambda_0>::operator()(int) const",
    );
    test(b"??R<lambda_1>@?0???R<lambda_0>@?0??PR26105@@YAHXZ@QBE@H@Z@QBE@H@Z", b"public: __thiscall `public: __thiscall `int __cdecl PR26105(void)'::`1'::<lambda_0>::operator()(int) const'::`1'::<lambda_1>::operator()(int) const");
    test(
        b"?unaligned_foo1@@YAPFAHXZ",
        b"int __unaligned * __cdecl unaligned_foo1(void)",
    );
    test(
        b"?unaligned_foo2@@YAPFAPFAHXZ",
        b"int __unaligned *__unaligned * __cdecl unaligned_foo2(void)",
    );
    test(
        b"?unaligned_foo3@@YAHXZ",
        b"int __cdecl unaligned_foo3(void)",
    );
    test(
        b"?unaligned_foo4@@YAXPFAH@Z",
        b"void __cdecl unaligned_foo4(int __unaligned *)",
    );
    test(
        b"?unaligned_foo5@@YAXPIFAH@Z",
        b"void __cdecl unaligned_foo5(int __unaligned *__restrict)",
    );
    test(
        b"??$unaligned_foo6@PAH@@YAPAHPAH@Z",
        b"int * __cdecl unaligned_foo6<int *>(int *)",
    );
    test(
        b"??$unaligned_foo6@PFAH@@YAPFAHPFAH@Z",
        b"int __unaligned * __cdecl unaligned_foo6<int __unaligned *>(int __unaligned *)",
    );
    test(
        b"?unaligned_foo8@unaligned_foo8_S@@QFCEXXZ",
        b"void __thiscall unaligned_foo8_S::unaligned_foo8(void) volatile __unaligned",
    );
    test(
        b"??R<lambda_1>@x@A@PR31197@@QBE@XZ",
        b"__thiscall PR31197::A::x::<lambda_1>::operator()(void) const",
    );
    test(
        b"?white@?1???R<lambda_1>@x@A@PR31197@@QBE@XZ@4HA",
        b"int `public: __thiscall PR31197::A::x::<lambda_1>::operator()(void) const'::`2'::white",
    );
    test(
        b"?f@@YAXW4<unnamed-enum-enumerator>@@@Z",
        b"void __cdecl f(enum <unnamed-enum-enumerator>)",
    );
}

#[test]
fn test_cxx14() {
    test(b"??$x@X@@3HA", b"int x<void>");
    test(
        b"?FunctionWithLocalType@@YA?A?<auto>@@XZ",
        b"<auto> __cdecl FunctionWithLocalType(void)",
    );
    test(b"?ValueFromFunctionWithLocalType@@3ULocalType@?1??FunctionWithLocalType@@YA?A?<auto>@@XZ@A", b"struct `<auto> __cdecl FunctionWithLocalType(void)'::`2'::LocalType ValueFromFunctionWithLocalType");
    test(
        b"??R<lambda_0>@@QBE?A?<auto>@@XZ",
        b"<auto> __thiscall <lambda_0>::operator()(void) const",
    );
    test(b"?ValueFromLambdaWithLocalType@@3ULocalType@?1???R<lambda_0>@@QBE?A?<auto>@@XZ@A", b"struct `public: <auto> __thiscall <lambda_0>::operator()(void) const'::`2'::LocalType ValueFromLambdaWithLocalType");
    test(b"?ValueFromTemplateFuncionWithLocalLambda@@3ULocalType@?2???R<lambda_1>@?0???$TemplateFuncionWithLocalLambda@H@@YA?A?<auto>@@H@Z@QBE?A?3@XZ@A", b"struct `public: <auto> __thiscall `<auto> __cdecl TemplateFuncionWithLocalLambda<int>(int)'::`1'::<lambda_1>::operator()(void) const'::`3'::LocalType ValueFromTemplateFuncionWithLocalLambda");
    test(
        b"??$TemplateFuncionWithLocalLambda@H@@YA?A?<auto>@@H@Z",
        b"<auto> __cdecl TemplateFuncionWithLocalLambda<int>(int)",
    );
    test(b"??R<lambda_1>@?0???$TemplateFuncionWithLocalLambda@H@@YA?A?<auto>@@H@Z@QBE?A?1@XZ", b"<auto> __thiscall `<auto> __cdecl TemplateFuncionWithLocalLambda<int>(int)'::`1'::<lambda_1>::operator()(void) const");
    test(b"??$WithPMD@$GA@A@?0@@3HA", b"int WithPMD<{0, 0, -1}>");
    test(
        b"?Zoo@@3U?$Foo@$1??$x@H@@3HA$1?1@3HA@@A",
        b"struct Foo<&int x<int>, &int x<int>> Zoo",
    );
    test(
        b"??$unaligned_x@PFAH@@3PFAHA",
        b"int __unaligned *unaligned_x<int __unaligned *>",
    );
}

#[test]
fn test_cxx17_noexcept() {
    test(b"?nochange@@YAXXZ", b"void __cdecl nochange(void)");
    test(b"?a@@YAXP6AHXZ@Z", b"void __cdecl a(int (__cdecl *)(void))");
    test(
        b"?a@@YAXP6AHX_E@Z",
        b"void __cdecl a(int (__cdecl *)(void) noexcept)",
    );
    test(b"?b@@YAXP6AHXZ@Z", b"void __cdecl b(int (__cdecl *)(void))");
    test(b"?c@@YAXP6AHXZ@Z", b"void __cdecl c(int (__cdecl *)(void))");
    test(
        b"?c@@YAXP6AHX_E@Z",
        b"void __cdecl c(int (__cdecl *)(void) noexcept)",
    );
    test(
        b"?ee@?$e@$$A6AXXZ@@EEAAXXZ",
        b"private: virtual void __cdecl e<void __cdecl(void)>::ee(void)",
    );
    test(
        b"?ee@?$e@$$A6AXX_E@@EEAAXXZ",
        b"private: virtual void __cdecl e<void __cdecl(void) noexcept>::ee(void)",
    );
}

#[test]
fn test_cxx20() {
    test(
        b"??__LA@@QEAA?AUno_suspend@@XZ",
        b"struct no_suspend __cdecl A::operator co_await(void)",
    );
    test(
        b"??__MS@@QEAA?AVstrong_ordering@std@@AEBU0@@Z'",
        b"class std::strong_ordering __cdecl S::operator<=>(struct S const &)",
    );
    test(b"?f@@YAX_Q@Z", b"void __cdecl f(char8_t)");
}

#[test]
fn test_mangle() {
    test(b"?a@@3HA", b"int a");
    test(b"?b@N@@3HA", b"int N::b");
    test(
        b"?anonymous@?A@N@@3HA",
        b"int N::`anonymous namespace'::anonymous",
    );
    test(
        b"?$RT1@NeedsReferenceTemporary@@3ABHB",
        b"int const &NeedsReferenceTemporary::$RT1",
    );
    test(
        b"?$RT1@NeedsReferenceTemporary@@3AEBHEB",
        b"int const &NeedsReferenceTemporary::$RT1",
    );
    test(b"?_c@@YAHXZ", b"int __cdecl _c(void)");
    test(b"?d@foo@@0FB", b"static short const foo::d");
    test(b"?e@foo@@1JC", b"static long volatile foo::e");
    test(b"?f@foo@@2DD", b"static char const volatile foo::f");
    test(b"??0foo@@QAE@XZ", b"__thiscall foo::foo(void)");
    test(b"??0foo@@QEAA@XZ", b"__cdecl foo::foo(void)");
    test(b"??1foo@@QAE@XZ", b"__thiscall foo::~foo(void)");
    test(b"??1foo@@QEAA@XZ", b"__cdecl foo::~foo(void)");
    test(b"??0foo@@QAE@H@Z", b"__thiscall foo::foo(int)");
    test(b"??0foo@@QEAA@H@Z", b"__cdecl foo::foo(int)");
    test(b"??0foo@@QAE@PAD@Z", b"__thiscall foo::foo(char *)");
    test(b"??0foo@@QEAA@PEAD@Z", b"__cdecl foo::foo(char *)");
    test(b"?bar@@YA?AVfoo@@XZ", b"class foo __cdecl bar(void)");
    test(b"?bar@@YA?AVfoo@@XZ", b"class foo __cdecl bar(void)");
    test(b"??Hfoo@@QAEHH@Z", b"int __thiscall foo::operator+(int)");
    test(b"??Hfoo@@QEAAHH@Z", b"int __cdecl foo::operator+(int)");
    test(
        b"??$?HH@S@@QEAAAEANH@Z",
        b"double & __cdecl S::operator+<int>(int)",
    );
    test(
        b"?static_method@foo@@SAPAV1@XZ",
        b"static class foo * __cdecl foo::static_method(void)",
    );
    test(
        b"?static_method@foo@@SAPEAV1@XZ",
        b"static class foo * __cdecl foo::static_method(void)",
    );
    test(b"?g@bar@@2HA", b"static int bar::g");
    test(b"?h1@@3QAHA", b"int *const h1");
    test(b"?h2@@3QBHB", b"int const *const h2");
    test(b"?h3@@3QIAHIA", b"int *const __restrict h3");
    test(b"?h3@@3QEIAHEIA", b"int *const __restrict h3");
    test(b"?i@@3PAY0BE@HA", b"int (*i)[20]");
    test(
        b"?FunArr@@3PAY0BE@P6AHHH@ZA",
        b"int (__cdecl *(*FunArr)[20])(int, int)",
    );
    test(
        b"?j@@3P6GHCE@ZA",
        b"int (__stdcall *j)(signed char, unsigned char)",
    );
    test(
        b"?funptr@@YAP6AHXZXZ",
        b"int (__cdecl * __cdecl funptr(void))(void)",
    );
    test(b"?m@@3PRfoo@@DR1@", b"char const foo::*m");
    test(b"?m@@3PERfoo@@DER1@", b"char const foo::*m");
    test(b"?k@@3PTfoo@@DT1@", b"char const volatile foo::*k");
    test(b"?k@@3PETfoo@@DET1@", b"char const volatile foo::*k");
    test(b"?l@@3P8foo@@AEHH@ZQ1@", b"int (__thiscall foo::*l)(int)");
    test(b"?g_cInt@@3HB", b"int const g_cInt");
    test(b"?g_vInt@@3HC", b"int volatile g_vInt");
    test(b"?g_cvInt@@3HD", b"int const volatile g_cvInt");
    test(
        b"?beta@@YI_N_J_W@Z",
        b"bool __fastcall beta(__int64, wchar_t)",
    );
    test(b"?beta@@YA_N_J_W@Z", b"bool __cdecl beta(__int64, wchar_t)");
    test(b"?alpha@@YGXMN@Z", b"void __stdcall alpha(float, double)");
    test(b"?alpha@@YAXMN@Z", b"void __cdecl alpha(float, double)");
    test(
        b"?gamma@@YAXVfoo@@Ubar@@Tbaz@@W4quux@@@Z",
        b"void __cdecl gamma(class foo, struct bar, union baz, enum quux)",
    );
    test(
        b"?gamma@@YAXVfoo@@Ubar@@Tbaz@@W4quux@@@Z",
        b"void __cdecl gamma(class foo, struct bar, union baz, enum quux)",
    );
    test(
        b"?delta@@YAXQAHABJ@Z",
        b"void __cdecl delta(int *const, long const &)",
    );
    test(
        b"?delta@@YAXQEAHAEBJ@Z",
        b"void __cdecl delta(int *const, long const &)",
    );
    test(
        b"?epsilon@@YAXQAY19BE@H@Z",
        b"void __cdecl epsilon(int (*const)[10][20])",
    );
    test(
        b"?epsilon@@YAXQEAY19BE@H@Z",
        b"void __cdecl epsilon(int (*const)[10][20])",
    );
    test(
        b"?zeta@@YAXP6AHHH@Z@Z",
        b"void __cdecl zeta(int (__cdecl *)(int, int))",
    );
    test(
        b"?zeta@@YAXP6AHHH@Z@Z",
        b"void __cdecl zeta(int (__cdecl *)(int, int))",
    );
    test(
        b"??2@YAPAXI@Z",
        b"void * __cdecl operator new(unsigned int)",
    );
    test(b"??3@YAXPAX@Z", b"void __cdecl operator delete(void *)");
    test(
        b"??_U@YAPAXI@Z",
        b"void * __cdecl operator new[](unsigned int)",
    );
    test(b"??_V@YAXPAX@Z", b"void __cdecl operator delete[](void *)");
    test(b"?color1@@3PANA", b"double *color1");
    test(b"?color2@@3QBNB", b"double const *const color2");
    test(b"?color3@@3QAY02$$CBNA", b"double const (*const color3)[3]");
    test(b"?color4@@3QAY02$$CBNA", b"double const (*const color4)[3]");
    test(
        b"?memptr1@@3RESB@@HES1@",
        b"int volatile B::*volatile memptr1",
    );
    test(b"?memptr2@@3PESB@@HES1@", b"int volatile B::*memptr2");
    test(b"?memptr3@@3REQB@@HEQ1@", b"int B::*volatile memptr3");
    test(
        b"?funmemptr1@@3RESB@@R6AHXZES1@",
        b"int (__cdecl *volatile B::*volatile funmemptr1)(void)",
    );
    test(
        b"?funmemptr2@@3PESB@@R6AHXZES1@",
        b"int (__cdecl *volatile B::*funmemptr2)(void)",
    );
    test(
        b"?funmemptr3@@3REQB@@P6AHXZEQ1@",
        b"int (__cdecl *B::*volatile funmemptr3)(void)",
    );
    test(
        b"?memptrtofun1@@3R8B@@EAAXXZEQ1@",
        b"void (__cdecl B::*volatile memptrtofun1)(void)",
    );
    test(
        b"?memptrtofun2@@3P8B@@EAAXXZEQ1@",
        b"void (__cdecl B::*memptrtofun2)(void)",
    );
    test(
        b"?memptrtofun3@@3P8B@@EAAXXZEQ1@",
        b"void (__cdecl B::*memptrtofun3)(void)",
    );
    test(
        b"?memptrtofun4@@3R8B@@EAAHXZEQ1@",
        b"int (__cdecl B::*volatile memptrtofun4)(void)",
    );
    test(
        b"?memptrtofun5@@3P8B@@EAA?CHXZEQ1@",
        b"int volatile (__cdecl B::*memptrtofun5)(void)",
    );
    test(
        b"?memptrtofun6@@3P8B@@EAA?BHXZEQ1@",
        b"int const (__cdecl B::*memptrtofun6)(void)",
    );
    test(
        b"?memptrtofun7@@3R8B@@EAAP6AHXZXZEQ1@",
        b"int (__cdecl * (__cdecl B::*volatile memptrtofun7)(void))(void)",
    );
    test(
        b"?memptrtofun8@@3P8B@@EAAR6AHXZXZEQ1@",
        b"int (__cdecl *volatile (__cdecl B::*memptrtofun8)(void))(void)",
    );
    test(
        b"?memptrtofun9@@3P8B@@EAAQ6AHXZXZEQ1@",
        b"int (__cdecl *const (__cdecl B::*memptrtofun9)(void))(void)",
    );
    test(b"?fooE@@YA?AW4E@@XZ", b"enum E __cdecl fooE(void)");
    test(b"?fooE@@YA?AW4E@@XZ", b"enum E __cdecl fooE(void)");
    test(b"?fooX@@YA?AVX@@XZ", b"class X __cdecl fooX(void)");
    test(b"?fooX@@YA?AVX@@XZ", b"class X __cdecl fooX(void)");
    test(b"?s0@PR13182@@3PADA", b"char *PR13182::s0");
    test(b"?s1@PR13182@@3PADA", b"char *PR13182::s1");
    test(b"?s2@PR13182@@3QBDB", b"char const *const PR13182::s2");
    test(b"?s3@PR13182@@3QBDB", b"char const *const PR13182::s3");
    test(
        b"?s4@PR13182@@3RCDC",
        b"char volatile *volatile PR13182::s4",
    );
    test(
        b"?s5@PR13182@@3SDDD",
        b"char const volatile *const volatile PR13182::s5",
    );
    test(b"?s6@PR13182@@3PBQBDB", b"char const *const *PR13182::s6");
    test(
        b"?local@?1??extern_c_func@@9@4HA",
        b"int `extern \"C\" extern_c_func'::`2'::local",
    );
    test(
        b"?local@?1??extern_c_func@@9@4HA",
        b"int `extern \"C\" extern_c_func'::`2'::local",
    );
    test(
        b"?v@?1??f@@YAHXZ@4U<unnamed-type-v>@?1??1@YAHXZ@A",
        b"struct `int __cdecl f(void)'::`2'::<unnamed-type-v> `int __cdecl f(void)'::`2'::v",
    );
    test(b"?v@?1???$f@H@@YAHXZ@4U<unnamed-type-v>@?1???$f@H@@YAHXZ@A", b"struct `int __cdecl f<int>(void)'::`2'::<unnamed-type-v> `int __cdecl f<int>(void)'::`2'::v");
    test(
        b"??2OverloadedNewDelete@@SAPAXI@Z",
        b"static void * __cdecl OverloadedNewDelete::operator new(unsigned int)",
    );
    test(
        b"??_UOverloadedNewDelete@@SAPAXI@Z",
        b"static void * __cdecl OverloadedNewDelete::operator new[](unsigned int)",
    );
    test(
        b"??3OverloadedNewDelete@@SAXPAX@Z",
        b"static void __cdecl OverloadedNewDelete::operator delete(void *)",
    );
    test(
        b"??_VOverloadedNewDelete@@SAXPAX@Z",
        b"static void __cdecl OverloadedNewDelete::operator delete[](void *)",
    );
    test(
        b"??HOverloadedNewDelete@@QAEHH@Z",
        b"int __thiscall OverloadedNewDelete::operator+(int)",
    );
    test(
        b"??2OverloadedNewDelete@@SAPEAX_K@Z",
        b"static void * __cdecl OverloadedNewDelete::operator new(unsigned __int64)",
    );
    test(
        b"??_UOverloadedNewDelete@@SAPEAX_K@Z",
        b"static void * __cdecl OverloadedNewDelete::operator new[](unsigned __int64)",
    );
    test(
        b"??3OverloadedNewDelete@@SAXPEAX@Z",
        b"static void __cdecl OverloadedNewDelete::operator delete(void *)",
    );
    test(
        b"??_VOverloadedNewDelete@@SAXPEAX@Z",
        b"static void __cdecl OverloadedNewDelete::operator delete[](void *)",
    );
    test(
        b"??HOverloadedNewDelete@@QEAAHH@Z",
        b"int __cdecl OverloadedNewDelete::operator+(int)",
    );
    test(
        b"??2TypedefNewDelete@@SAPAXI@Z",
        b"static void * __cdecl TypedefNewDelete::operator new(unsigned int)",
    );
    test(
        b"??_UTypedefNewDelete@@SAPAXI@Z",
        b"static void * __cdecl TypedefNewDelete::operator new[](unsigned int)",
    );
    test(
        b"??3TypedefNewDelete@@SAXPAX@Z",
        b"static void __cdecl TypedefNewDelete::operator delete(void *)",
    );
    test(
        b"??_VTypedefNewDelete@@SAXPAX@Z",
        b"static void __cdecl TypedefNewDelete::operator delete[](void *)",
    );
    test(
        b"?vector_func@@YQXXZ",
        b"void __vectorcall vector_func(void)",
    );
    test(
        b"?swift_func@@YSXXZ",
        b"void __attribute__((__swiftcall__)) swift_func(void)",
    );
    test(
        b"?swift_async_func@@YWXXZ",
        b"void __attribute__((__swiftasynccall__)) swift_async_func(void)",
    );
    test(
        b"??$fn_tmpl@$1?extern_c_func@@YAXXZ@@YAXXZ",
        b"void __cdecl fn_tmpl<&void __cdecl extern_c_func(void)>(void)",
    );
    test(
        b"?overloaded_fn@@$$J0YAXXZ",
        b"extern \"C\" void __cdecl overloaded_fn(void)",
    );
    test(
        b"?f@UnnamedType@@YAXQAPAU<unnamed-type-T1>@S@1@@Z",
        b"void __cdecl UnnamedType::f(struct UnnamedType::S::<unnamed-type-T1> **const)",
    );
    test(
        b"?f@UnnamedType@@YAXUT2@S@1@@Z",
        b"void __cdecl UnnamedType::f(struct UnnamedType::S::T2)",
    );
    test(
        b"?f@UnnamedType@@YAXPAUT4@S@1@@Z",
        b"void __cdecl UnnamedType::f(struct UnnamedType::S::T4 *)",
    );
    test(
        b"?f@UnnamedType@@YAXUT4@S@1@@Z",
        b"void __cdecl UnnamedType::f(struct UnnamedType::S::T4)",
    );
    test(
        b"?f@UnnamedType@@YAXUT5@S@1@@Z",
        b"void __cdecl UnnamedType::f(struct UnnamedType::S::T5)",
    );
    test(
        b"?f@UnnamedType@@YAXUT2@S@1@@Z",
        b"void __cdecl UnnamedType::f(struct UnnamedType::S::T2)",
    );
    test(
        b"?f@UnnamedType@@YAXUT4@S@1@@Z",
        b"void __cdecl UnnamedType::f(struct UnnamedType::S::T4)",
    );
    test(
        b"?f@UnnamedType@@YAXUT5@S@1@@Z",
        b"void __cdecl UnnamedType::f(struct UnnamedType::S::T5)",
    );
    test(
        b"?f@Atomic@@YAXU?$_Atomic@H@__clang@@@Z",
        b"void __cdecl Atomic::f(struct __clang::_Atomic<int>)",
    );
    test(
        b"?f@Complex@@YAXU?$_Complex@H@__clang@@@Z",
        b"void __cdecl Complex::f(struct __clang::_Complex<int>)",
    );
    test(
        b"?f@Float16@@YAXU_Float16@__clang@@@Z",
        b"void __cdecl Float16::f(struct __clang::_Float16)",
    );
    test(b"??0?$L@H@NS@@QEAA@XZ", b"__cdecl NS::L<int>::L<int>(void)");
    test(b"??0Bar@Foo@@QEAA@XZ", b"__cdecl Foo::Bar::Bar(void)");
    test(
        b"??0?$L@V?$H@PAH@PR26029@@@PR26029@@QAE@XZ",
        b"__thiscall PR26029::L<class PR26029::H<int *>>::L<class PR26029::H<int *>>(void)",
    );
    test(b"??$emplace_back@ABH@?$vector@HV?$allocator@H@std@@@std@@QAE?A?<decltype-auto>@@ABH@Z", b"<decltype-auto> __thiscall std::vector<int, class std::allocator<int>>::emplace_back<int const &>(int const &)");
    test(
        b"?pub_foo@S@@QAEXXZ",
        b"public: void __thiscall S::pub_foo(void)",
    );
    test(
        b"?pub_stat_foo@S@@SAXXZ",
        b"public: static void __cdecl S::pub_stat_foo(void)",
    );
    test(
        b"?pub_virt_foo@S@@UAEXXZ",
        b"public: virtual void __thiscall S::pub_virt_foo(void)",
    );
    test(
        b"?prot_foo@S@@IAEXXZ",
        b"protected: void __thiscall S::prot_foo(void)",
    );
    test(
        b"?prot_stat_foo@S@@KAXXZ",
        b"protected: static void __cdecl S::prot_stat_foo(void)",
    );
    test(
        b"?prot_virt_foo@S@@MAEXXZ",
        b"protected: virtual void __thiscall S::prot_virt_foo(void)",
    );
    test(
        b"?priv_foo@S@@AAEXXZ",
        b"private: void __thiscall S::priv_foo(void)",
    );
    test(
        b"?priv_stat_foo@S@@CAXXZ",
        b"private: static void __cdecl S::priv_stat_foo(void)",
    );
    test(
        b"?priv_virt_foo@S@@EAEXXZ",
        b"private: virtual void __thiscall S::priv_virt_foo(void)",
    );
}

#[test]
fn test_md5() {
    test(
        b"??@a6a285da2eea70dba6b578022be61d81@",
        b"??@a6a285da2eea70dba6b578022be61d81@",
    );
    test(
        b"??@a6a285da2eea70dba6b578022be61d81@asdf",
        b"??@a6a285da2eea70dba6b578022be61d81@",
    );
    test(
        b"??@a6a285da2eea70dba6b578022be61d81@??_R4@",
        b"??@a6a285da2eea70dba6b578022be61d81@??_R4@",
    );
}

#[test]
fn test_nested_scopes() {
    test(b"?M@?@??L@@YAHXZ@4HA", b"int `int __cdecl L(void)'::`0'::M");
    test(b"?M@?0??L@@YAHXZ@4HA", b"int `int __cdecl L(void)'::`1'::M");
    test(b"?M@?1??L@@YAHXZ@4HA", b"int `int __cdecl L(void)'::`2'::M");
    test(b"?M@?2??L@@YAHXZ@4HA", b"int `int __cdecl L(void)'::`3'::M");
    test(b"?M@?3??L@@YAHXZ@4HA", b"int `int __cdecl L(void)'::`4'::M");
    test(b"?M@?4??L@@YAHXZ@4HA", b"int `int __cdecl L(void)'::`5'::M");
    test(b"?M@?5??L@@YAHXZ@4HA", b"int `int __cdecl L(void)'::`6'::M");
    test(b"?M@?6??L@@YAHXZ@4HA", b"int `int __cdecl L(void)'::`7'::M");
    test(b"?M@?7??L@@YAHXZ@4HA", b"int `int __cdecl L(void)'::`8'::M");
    test(b"?M@?8??L@@YAHXZ@4HA", b"int `int __cdecl L(void)'::`9'::M");
    test(
        b"?M@?9??L@@YAHXZ@4HA",
        b"int `int __cdecl L(void)'::`10'::M",
    );
    test(
        b"?M@?L@??L@@YAHXZ@4HA",
        b"int `int __cdecl L(void)'::`11'::M",
    );
    test(
        b"?M@?M@??L@@YAHXZ@4HA",
        b"int `int __cdecl L(void)'::`12'::M",
    );
    test(
        b"?M@?N@??L@@YAHXZ@4HA",
        b"int `int __cdecl L(void)'::`13'::M",
    );
    test(
        b"?M@?O@??L@@YAHXZ@4HA",
        b"int `int __cdecl L(void)'::`14'::M",
    );
    test(
        b"?M@?P@??L@@YAHXZ@4HA",
        b"int `int __cdecl L(void)'::`15'::M",
    );
    test(
        b"?M@?BA@??L@@YAHXZ@4HA",
        b"int `int __cdecl L(void)'::`16'::M",
    );
    test(
        b"?M@?BB@??L@@YAHXZ@4HA",
        b"int `int __cdecl L(void)'::`17'::M",
    );
    test(
        b"?j@?1??L@@YAHXZ@4UJ@@A",
        b"struct J `int __cdecl L(void)'::`2'::j",
    );
    test(b"?NN@0XX@@3HA", b"int XX::NN::NN");
    test(b"?MM@0NN@XX@@3HA", b"int XX::NN::MM::MM");
    test(b"?NN@MM@0XX@@3HA", b"int XX::NN::MM::NN");
    test(b"?OO@0NN@01XX@@3HA", b"int XX::NN::OO::NN::OO::OO");
    test(b"?NN@OO@010XX@@3HA", b"int XX::NN::OO::NN::OO::NN");
    test(b"?M@?1??0@YAHXZ@4HA", b"int `int __cdecl M(void)'::`2'::M");
    test(
        b"?L@?2??M@0?2??0@YAHXZ@QEAAHXZ@4HA",
        b"int `public: int __cdecl `int __cdecl L(void)'::`3'::L::M(void)'::`3'::L",
    );
    test(
        b"?M@?2??0L@?2??1@YAHXZ@QEAAHXZ@4HA",
        b"int `public: int __cdecl `int __cdecl L(void)'::`3'::L::M(void)'::`3'::M",
    );
    test(
        b"?M@?1???$L@H@@YAHXZ@4HA",
        b"int `int __cdecl L<int>(void)'::`2'::M",
    );
    test(
        b"?SN@?$NS@H@NS@@QEAAHXZ",
        b"int __cdecl NS::NS<int>::SN(void)",
    );
    test(
        b"?NS@?1??SN@?$NS@H@0@QEAAHXZ@4HA",
        b"int `public: int __cdecl NS::NS<int>::SN(void)'::`2'::NS",
    );
    test(
        b"?SN@?1??0?$NS@H@NS@@QEAAHXZ@4HA",
        b"int `public: int __cdecl NS::NS<int>::SN(void)'::`2'::SN",
    );
    test(
        b"?NS@?1??SN@?$NS@H@10@QEAAHXZ@4HA",
        b"int `public: int __cdecl NS::SN::NS<int>::SN(void)'::`2'::NS",
    );
    test(
        b"?SN@?1??0?$NS@H@0NS@@QEAAHXZ@4HA",
        b"int `public: int __cdecl NS::SN::NS<int>::SN(void)'::`2'::SN",
    );
    test(b"?X@?$C@H@C@0@2HB", b"static int const X::C::C<int>::X");
    test(
        b"?X@?$C@H@C@1@2HB",
        b"static int const C<int>::C::C<int>::X",
    );
    test(b"?X@?$C@H@C@2@2HB", b"static int const C::C::C<int>::X");
    test(b"?C@?1??B@?$C@H@0101A@@QEAAHXZ@4U201013@A", b"struct A::B::C::B::C::C<int> `public: int __cdecl A::B::C::B::C::C<int>::B(void)'::`2'::C");
    test(
        b"?B@?1??0?$C@H@C@020A@@QEAAHXZ@4HA",
        b"int `public: int __cdecl A::B::C::B::C::C<int>::B(void)'::`2'::B",
    );
    test(
        b"?A@?1??B@?$C@H@C@1310@QEAAHXZ@4HA",
        b"int `public: int __cdecl A::B::C::B::C::C<int>::B(void)'::`2'::A",
    );
    test(b"?a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@a@@3HA", b"int a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a::a");
}

#[test]
fn test_operators() {
    test(b"??0Base@@QEAA@XZ", b"__cdecl Base::Base(void)");
    test(b"??1Base@@UEAA@XZ", b"virtual __cdecl Base::~Base(void)");
    test(
        b"??2@YAPEAX_K@Z",
        b"void * __cdecl operator new(unsigned __int64)",
    );
    test(
        b"??3@YAXPEAX_K@Z",
        b"void __cdecl operator delete(void *, unsigned __int64)",
    );
    test(b"??4Base@@QEAAHH@Z", b"int __cdecl Base::operator=(int)");
    test(b"??6Base@@QEAAHH@Z", b"int __cdecl Base::operator<<(int)");
    test(b"??5Base@@QEAAHH@Z", b"int __cdecl Base::operator>>(int)");
    test(b"??7Base@@QEAAHXZ", b"int __cdecl Base::operator!(void)");
    test(b"??8Base@@QEAAHH@Z", b"int __cdecl Base::operator==(int)");
    test(b"??9Base@@QEAAHH@Z", b"int __cdecl Base::operator!=(int)");
    test(b"??ABase@@QEAAHH@Z", b"int __cdecl Base::operator[](int)");
    test(b"??BBase@@QEAAHXZ", b"__cdecl Base::operator int(void)");
    test(b"??CBase@@QEAAHXZ", b"int __cdecl Base::operator->(void)");
    test(b"??DBase@@QEAAHXZ", b"int __cdecl Base::operator*(void)");
    test(b"??EBase@@QEAAHXZ", b"int __cdecl Base::operator++(void)");
    test(b"??EBase@@QEAAHH@Z", b"int __cdecl Base::operator++(int)");
    test(b"??FBase@@QEAAHXZ", b"int __cdecl Base::operator--(void)");
    test(b"??FBase@@QEAAHH@Z", b"int __cdecl Base::operator--(int)");
    test(b"??GBase@@QEAAHH@Z", b"int __cdecl Base::operator-(int)");
    test(b"??HBase@@QEAAHH@Z", b"int __cdecl Base::operator+(int)");
    test(b"??IBase@@QEAAHH@Z", b"int __cdecl Base::operator&(int)");
    test(b"??JBase@@QEAAHH@Z", b"int __cdecl Base::operator->*(int)");
    test(b"??KBase@@QEAAHH@Z", b"int __cdecl Base::operator/(int)");
    test(b"??LBase@@QEAAHH@Z", b"int __cdecl Base::operator%(int)");
    test(b"??MBase@@QEAAHH@Z", b"int __cdecl Base::operator<(int)");
    test(b"??NBase@@QEAAHH@Z", b"int __cdecl Base::operator<=(int)");
    test(b"??OBase@@QEAAHH@Z", b"int __cdecl Base::operator>(int)");
    test(b"??PBase@@QEAAHH@Z", b"int __cdecl Base::operator>=(int)");
    test(b"??QBase@@QEAAHH@Z", b"int __cdecl Base::operator,(int)");
    test(b"??RBase@@QEAAHXZ", b"int __cdecl Base::operator()(void)");
    test(b"??SBase@@QEAAHXZ", b"int __cdecl Base::operator~(void)");
    test(b"??TBase@@QEAAHH@Z", b"int __cdecl Base::operator^(int)");
    test(b"??UBase@@QEAAHH@Z", b"int __cdecl Base::operator|(int)");
    test(b"??VBase@@QEAAHH@Z", b"int __cdecl Base::operator&&(int)");
    test(b"??WBase@@QEAAHH@Z", b"int __cdecl Base::operator||(int)");
    test(b"??XBase@@QEAAHH@Z", b"int __cdecl Base::operator*=(int)");
    test(b"??YBase@@QEAAHH@Z", b"int __cdecl Base::operator+=(int)");
    test(b"??ZBase@@QEAAHH@Z", b"int __cdecl Base::operator-=(int)");
    test(b"??_0Base@@QEAAHH@Z", b"int __cdecl Base::operator/=(int)");
    test(b"??_1Base@@QEAAHH@Z", b"int __cdecl Base::operator%=(int)");
    test(b"??_2Base@@QEAAHH@Z", b"int __cdecl Base::operator>>=(int)");
    test(b"??_3Base@@QEAAHH@Z", b"int __cdecl Base::operator<<=(int)");
    test(b"??_4Base@@QEAAHH@Z", b"int __cdecl Base::operator&=(int)");
    test(b"??_5Base@@QEAAHH@Z", b"int __cdecl Base::operator|=(int)");
    test(b"??_6Base@@QEAAHH@Z", b"int __cdecl Base::operator^=(int)");
    test(b"??_7Base@@6B@", b"const Base::`vftable'");
    test(b"??_7A@B@@6BC@D@@@", b"const B::A::`vftable'{for `D::C'}");
    test(b"??_8Middle2@@7B@", b"const Middle2::`vbtable'");
    test(
        b"??_9Base@@$B7AA",
        b"[thunk]: __cdecl Base::`vcall'{8, {flat}}",
    );
    test(
        b"??_B?1??getS@@YAAAUS@@XZ@51",
        b"`struct S & __cdecl getS(void)'::`2'::`local static guard'{2}",
    );
    test(b"??_C@_02PCEFGMJL@hi?$AA@", b"\"hi\"");
    test(
        b"??_DDiamond@@QEAAXXZ",
        b"void __cdecl Diamond::`vbase dtor'(void)",
    );
    test(
        b"??_EBase@@UEAAPEAXI@Z",
        b"virtual void * __cdecl Base::`vector deleting dtor'(unsigned int)",
    );
    test(b"??_EBase@@G3AEPAXI@Z", b"[thunk]: private: void * __thiscall Base::`vector deleting dtor'`adjustor{4}'(unsigned int)");
    test(
        b"??_F?$SomeTemplate@H@@QAEXXZ",
        b"void __thiscall SomeTemplate<int>::`default ctor closure'(void)",
    );
    test(
        b"??_GBase@@UEAAPEAXI@Z",
        b"virtual void * __cdecl Base::`scalar deleting dtor'(unsigned int)",
    );
    test(b"??_H@YAXPEAX_K1P6APEAX0@Z@Z", b"void __cdecl `vector ctor iterator'(void *, unsigned __int64, unsigned __int64, void * (__cdecl *)(void *))");
    test(b"??_I@YAXPEAX_K1P6AX0@Z@Z", b"void __cdecl `vector dtor iterator'(void *, unsigned __int64, unsigned __int64, void (__cdecl *)(void *))");
    test(
        b"??_JBase@@UEAAPEAXI@Z",
        b"virtual void * __cdecl Base::`vector vbase ctor iterator'(unsigned int)",
    );
    test(
        b"??_KBase@@UEAAPEAXI@Z",
        b"virtual void * __cdecl Base::`virtual displacement map'(unsigned int)",
    );
    test(
        b"??_LBase@@UEAAPEAXI@Z",
        b"virtual void * __cdecl Base::`eh vector ctor iterator'(unsigned int)",
    );
    test(
        b"??_MBase@@UEAAPEAXI@Z",
        b"virtual void * __cdecl Base::`eh vector dtor iterator'(unsigned int)",
    );
    test(
        b"??_NBase@@UEAAPEAXI@Z",
        b"virtual void * __cdecl Base::`eh vector vbase ctor iterator'(unsigned int)",
    );
    test(
        b"??_O?$SomeTemplate@H@@QAEXXZ",
        b"void __thiscall SomeTemplate<int>::`copy ctor closure'(void)",
    );
    test(b"??_SBase@@6B@", b"const Base::`local vftable'");
    test(
        b"??_TDerived@@QEAAXXZ",
        b"void __cdecl Derived::`local vftable ctor closure'(void)",
    );
    test(
        b"??_U@YAPEAX_KAEAVklass@@@Z",
        b"void * __cdecl operator new[](unsigned __int64, class klass &)",
    );
    test(
        b"??_V@YAXPEAXAEAVklass@@@Z",
        b"void __cdecl operator delete[](void *, class klass &)",
    );
    test(b"??_R0?AUBase@@@8", b"struct Base `RTTI Type Descriptor'");
    test(b".?AUBase@@", b"struct Base `RTTI Type Descriptor Name'");
    test(
        b"??_R1A@?0A@EA@Base@@8",
        b"Base::`RTTI Base Class Descriptor at (0, -1, 0, 64)'",
    );
    test(b"??_R2Base@@8", b"Base::`RTTI Base Class Array'");
    test(b"??_R3Base@@8", b"Base::`RTTI Class Hierarchy Descriptor'");
    test(
        b"??_R4Base@@6B@",
        b"const Base::`RTTI Complete Object Locator'",
    );
    test(
        b"??__EFoo@@YAXXZ",
        b"void __cdecl `dynamic initializer for 'Foo''(void)",
    );
    test(
        b"??__E?i@C@@0HA@@YAXXZ",
        b"void __cdecl `dynamic initializer for `private: static int C::i''(void)",
    );
    test(
        b"??__FFoo@@YAXXZ",
        b"void __cdecl `dynamic atexit destructor for 'Foo''(void)",
    );
    test(b"??__F_decisionToDFA@XPathLexer@@0V?$vector@VDFA@dfa@antlr4@@V?$allocator@VDFA@dfa@antlr4@@@std@@@std@@A@YAXXZ", b"void __cdecl `dynamic atexit destructor for `private: static class std::vector<class antlr4::dfa::DFA, class std::allocator<class antlr4::dfa::DFA>> XPathLexer::_decisionToDFA''(void)");
    test(
        b"??__J?1??f@@YAAAUS@@XZ@51",
        b"`struct S & __cdecl f(void)'::`2'::`local static thread guard'{2}",
    );
    test(
        b"??__K_deg@@YAHO@Z",
        b"int __cdecl operator \"\"_deg(long double)",
    );
}

#[test]
fn test_options() {
    let test_options = |mangled_name: &[u8],
                        default: &[u8],
                        no_calling_conv: &[u8],
                        no_return: &[u8],
                        no_access: &[u8],
                        no_member_type: &[u8],
                        no_variable_type: &[u8],
                        no_all: &[u8]| {
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
        do_test(mangled_name, no_all, false, Flags::all());
    };

    test_options(
        b"?func@MyClass@@UEAAHHH@Z",
        b"public: virtual int __cdecl MyClass::func(int, int)",
        b"public: virtual int MyClass::func(int, int)",
        b"public: virtual __cdecl MyClass::func(int, int)",
        b"virtual int __cdecl MyClass::func(int, int)",
        b"public: int __cdecl MyClass::func(int, int)",
        b"public: virtual int __cdecl MyClass::func(int, int)",
        b"MyClass::func(int, int)",
    );
    test_options(
        b"?array2d@@3PAY09HA",
        b"int (*array2d)[10]",
        b"int (*array2d)[10]",
        b"int (*array2d)[10]",
        b"int (*array2d)[10]",
        b"int (*array2d)[10]",
        b"array2d",
        b"array2d",
    );
    test_options(
        b"?a@abc@@3PAY09HA",
        b"int (*abc::a)[10]",
        b"int (*abc::a)[10]",
        b"int (*abc::a)[10]",
        b"int (*abc::a)[10]",
        b"int (*abc::a)[10]",
        b"abc::a",
        b"abc::a",
    );
    test_options(
        b"?x@@3PEAEEA",
        b"unsigned char *x",
        b"unsigned char *x",
        b"unsigned char *x",
        b"unsigned char *x",
        b"unsigned char *x",
        b"x",
        b"x",
    );
}

#[test]
fn test_return_qualifiers() {
    test(b"?a1@@YAXXZ", b"void __cdecl a1(void)");
    test(b"?a2@@YAHXZ", b"int __cdecl a2(void)");
    test(b"?a3@@YA?BHXZ", b"int const __cdecl a3(void)");
    test(b"?a4@@YA?CHXZ", b"int volatile __cdecl a4(void)");
    test(b"?a5@@YA?DHXZ", b"int const volatile __cdecl a5(void)");
    test(b"?a6@@YAMXZ", b"float __cdecl a6(void)");
    test(b"?b1@@YAPAHXZ", b"int * __cdecl b1(void)");
    test(b"?b2@@YAPBDXZ", b"char const * __cdecl b2(void)");
    test(b"?b3@@YAPAMXZ", b"float * __cdecl b3(void)");
    test(b"?b4@@YAPBMXZ", b"float const * __cdecl b4(void)");
    test(b"?b5@@YAPCMXZ", b"float volatile * __cdecl b5(void)");
    test(b"?b6@@YAPDMXZ", b"float const volatile * __cdecl b6(void)");
    test(b"?b7@@YAAAMXZ", b"float & __cdecl b7(void)");
    test(b"?b8@@YAABMXZ", b"float const & __cdecl b8(void)");
    test(b"?b9@@YAACMXZ", b"float volatile & __cdecl b9(void)");
    test(
        b"?b10@@YAADMXZ",
        b"float const volatile & __cdecl b10(void)",
    );
    test(b"?b11@@YAPAPBDXZ", b"char const ** __cdecl b11(void)");
    test(b"?c1@@YA?AVA@@XZ", b"class A __cdecl c1(void)");
    test(b"?c2@@YA?BVA@@XZ", b"class A const __cdecl c2(void)");
    test(b"?c3@@YA?CVA@@XZ", b"class A volatile __cdecl c3(void)");
    test(
        b"?c4@@YA?DVA@@XZ",
        b"class A const volatile __cdecl c4(void)",
    );
    test(b"?c5@@YAPBVA@@XZ", b"class A const * __cdecl c5(void)");
    test(b"?c6@@YAPCVA@@XZ", b"class A volatile * __cdecl c6(void)");
    test(
        b"?c7@@YAPDVA@@XZ",
        b"class A const volatile * __cdecl c7(void)",
    );
    test(b"?c8@@YAAAVA@@XZ", b"class A & __cdecl c8(void)");
    test(b"?c9@@YAABVA@@XZ", b"class A const & __cdecl c9(void)");
    test(b"?c10@@YAACVA@@XZ", b"class A volatile & __cdecl c10(void)");
    test(
        b"?c11@@YAADVA@@XZ",
        b"class A const volatile & __cdecl c11(void)",
    );
    test(b"?d1@@YA?AV?$B@H@@XZ", b"class B<int> __cdecl d1(void)");
    test(
        b"?d2@@YA?AV?$B@PBD@@XZ",
        b"class B<char const *> __cdecl d2(void)",
    );
    test(
        b"?d3@@YA?AV?$B@VA@@@@XZ",
        b"class B<class A> __cdecl d3(void)",
    );
    test(
        b"?d4@@YAPAV?$B@VA@@@@XZ",
        b"class B<class A> * __cdecl d4(void)",
    );
    test(
        b"?d5@@YAPBV?$B@VA@@@@XZ",
        b"class B<class A> const * __cdecl d5(void)",
    );
    test(
        b"?d6@@YAPCV?$B@VA@@@@XZ",
        b"class B<class A> volatile * __cdecl d6(void)",
    );
    test(
        b"?d7@@YAPDV?$B@VA@@@@XZ",
        b"class B<class A> const volatile * __cdecl d7(void)",
    );
    test(
        b"?d8@@YAAAV?$B@VA@@@@XZ",
        b"class B<class A> & __cdecl d8(void)",
    );
    test(
        b"?d9@@YAABV?$B@VA@@@@XZ",
        b"class B<class A> const & __cdecl d9(void)",
    );
    test(
        b"?d10@@YAACV?$B@VA@@@@XZ",
        b"class B<class A> volatile & __cdecl d10(void)",
    );
    test(
        b"?d11@@YAADV?$B@VA@@@@XZ",
        b"class B<class A> const volatile & __cdecl d11(void)",
    );
    test(b"?e1@@YA?AW4Enum@@XZ", b"Enum __cdecl e1(void)");
    test(b"?e2@@YA?BW4Enum@@XZ", b"Enum const __cdecl e2(void)");
    test(b"?e3@@YAPAW4Enum@@XZ", b"Enum * __cdecl e3(void)");
    test(b"?e4@@YAAAW4Enum@@XZ", b"Enum & __cdecl e4(void)");
    test(b"?f1@@YA?AUS@@XZ", b"struct S __cdecl f1(void)");
    test(b"?f2@@YA?BUS@@XZ", b"struct S const __cdecl f2(void)");
    test(b"?f3@@YAPAUS@@XZ", b"struct S * __cdecl f3(void)");
    test(b"?f4@@YAPBUS@@XZ", b"struct S const * __cdecl f4(void)");
    test(
        b"?f5@@YAPDUS@@XZ",
        b"struct S const volatile * __cdecl f5(void)",
    );
    test(b"?f6@@YAAAUS@@XZ", b"struct S & __cdecl f6(void)");
    test(b"?f7@@YAQAUS@@XZ", b"struct S *const __cdecl f7(void)");
    test(b"?f8@@YAPQS@@HXZ", b"int S::* __cdecl f8(void)");
    test(b"?f9@@YAQQS@@HXZ", b"int S::*const __cdecl f9(void)");
    test(
        b"?f10@@YAPIQS@@HXZ",
        b"int S::*__restrict __cdecl f10(void)",
    );
    test(
        b"?f11@@YAQIQS@@HXZ",
        b"int S::*const __restrict __cdecl f11(void)",
    );
    test(
        b"?g1@@YAP6AHH@ZXZ",
        b"int (__cdecl * __cdecl g1(void))(int)",
    );
    test(
        b"?g2@@YAQ6AHH@ZXZ",
        b"int (__cdecl *const __cdecl g2(void))(int)",
    );
    test(
        b"?g3@@YAPAP6AHH@ZXZ",
        b"int (__cdecl ** __cdecl g3(void))(int)",
    );
    test(
        b"?g4@@YAPBQ6AHH@ZXZ",
        b"int (__cdecl *const * __cdecl g4(void))(int)",
    );
    test(b"?h1@@YAAIAHXZ", b"int &__restrict __cdecl h1(void)");
}

#[test]
fn test_string_literals() {
    {
        let inputs: [&[u8]; 256] = [
            b"??_C@_01CNACBAHC@?$PP?$AA@",
            b"??_C@_01DEBJCBDD@?$PO?$AA@",
            b"??_C@_01BPDEHCPA@?$PN?$AA@",
            b"??_C@_01GCPEDLB@?$PM?$AA@",
            b"??_C@_01EJGONFHG@?$PL?$AA@",
            b"??_C@_01FAHFOEDH@?z?$AA@",
            b"??_C@_01HLFILHPE@?y?$AA@",
            b"??_C@_01GCEDIGLF@?x?$AA@",
            b"??_C@_01OFNLJKHK@?w?$AA@",
            b"??_C@_01PMMAKLDL@?v?$AA@",
            b"??_C@_01NHONPIPI@?u?$AA@",
            b"??_C@_01MOPGMJLJ@?t?$AA@",
            b"??_C@_01IBLHFPHO@?s?$AA@",
            b"??_C@_01JIKMGODP@?r?$AA@",
            b"??_C@_01LDIBDNPM@?q?$AA@",
            b"??_C@_01KKJKAMLN@?p?$AA@",
            b"??_C@_01GHMAACCD@?o?$AA@",
            b"??_C@_01HONLDDGC@?n?$AA@",
            b"??_C@_01FFPGGAKB@?m?$AA@",
            b"??_C@_01EMONFBOA@?l?$AA@",
            b"??_C@_01DKMMHCH@?k?$AA@",
            b"??_C@_01BKLHPGGG@?j?$AA@",
            b"??_C@_01DBJKKFKF@?i?$AA@",
            b"??_C@_01CIIBJEOE@?h?$AA@",
            b"??_C@_01KPBJIICL@?g?$AA@",
            b"??_C@_01LGACLJGK@?f?$AA@",
            b"??_C@_01JNCPOKKJ@?e?$AA@",
            b"??_C@_01IEDENLOI@?d?$AA@",
            b"??_C@_01MLHFENCP@?c?$AA@",
            b"??_C@_01NCGOHMGO@?b?$AA@",
            b"??_C@_01PJEDCPKN@?a?$AA@",
            b"??_C@_01OAFIBOOM@?$OA?$AA@",
            b"??_C@_01LIIGDENA@?$NP?$AA@",
            b"??_C@_01KBJNAFJB@?$NO?$AA@",
            b"??_C@_01IKLAFGFC@?$NN?$AA@",
            b"??_C@_01JDKLGHBD@?$NM?$AA@",
            b"??_C@_01NMOKPBNE@?$NL?$AA@",
            b"??_C@_01MFPBMAJF@?Z?$AA@",
            b"??_C@_01OONMJDFG@?Y?$AA@",
            b"??_C@_01PHMHKCBH@?X?$AA@",
            b"??_C@_01HAFPLONI@?W?$AA@",
            b"??_C@_01GJEEIPJJ@?V?$AA@",
            b"??_C@_01ECGJNMFK@?U?$AA@",
            b"??_C@_01FLHCONBL@?T?$AA@",
            b"??_C@_01BEDDHLNM@?S?$AA@",
            b"??_C@_01NCIEKJN@?R?$AA@",
            b"??_C@_01CGAFBJFO@?Q?$AA@",
            b"??_C@_01DPBOCIBP@?P?$AA@",
            b"??_C@_01PCEECGIB@?O?$AA@",
            b"??_C@_01OLFPBHMA@?N?$AA@",
            b"??_C@_01MAHCEEAD@?M?$AA@",
            b"??_C@_01NJGJHFEC@?L?$AA@",
            b"??_C@_01JGCIODIF@?K?$AA@",
            b"??_C@_01IPDDNCME@?J?$AA@",
            b"??_C@_01KEBOIBAH@?I?$AA@",
            b"??_C@_01LNAFLAEG@?H?$AA@",
            b"??_C@_01DKJNKMIJ@?G?$AA@",
            b"??_C@_01CDIGJNMI@?F?$AA@",
            b"??_C@_01IKLMOAL@?E?$AA@",
            b"??_C@_01BBLAPPEK@?D?$AA@",
            b"??_C@_01FOPBGJIN@?C?$AA@",
            b"??_C@_01EHOKFIMM@?B?$AA@",
            b"??_C@_01GMMHALAP@?A?$AA@",
            b"??_C@_01HFNMDKEO@?$MA?$AA@",
            b"??_C@_01NNHLFPHH@?$LP?$AA@",
            b"??_C@_01MEGAGODG@?$LO?$AA@",
            b"??_C@_01OPENDNPF@?$LN?$AA@",
            b"??_C@_01PGFGAMLE@?$LM?$AA@",
            b"??_C@_01LJBHJKHD@?$LL?$AA@",
            b"??_C@_01KAAMKLDC@?$LK?$AA@",
            b"??_C@_01ILCBPIPB@?$LJ?$AA@",
            b"??_C@_01JCDKMJLA@?$LI?$AA@",
            b"??_C@_01BFKCNFHP@?$LH?$AA@",
            b"??_C@_01MLJOEDO@?$LG?$AA@",
            b"??_C@_01CHJELHPN@?$LF?$AA@",
            b"??_C@_01DOIPIGLM@?$LE?$AA@",
            b"??_C@_01HBMOBAHL@?$LD?$AA@",
            b"??_C@_01GINFCBDK@?$LC?$AA@",
            b"??_C@_01EDPIHCPJ@?$LB?$AA@",
            b"??_C@_01FKODEDLI@?$LA?$AA@",
            b"??_C@_01JHLJENCG@?$KP?$AA@",
            b"??_C@_01IOKCHMGH@?$KO?$AA@",
            b"??_C@_01KFIPCPKE@?$KN?$AA@",
            b"??_C@_01LMJEBOOF@?$KM?$AA@",
            b"??_C@_01PDNFIICC@?$KL?$AA@",
            b"??_C@_01OKMOLJGD@?$KK?$AA@",
            b"??_C@_01MBODOKKA@?$KJ?$AA@",
            b"??_C@_01NIPINLOB@?$KI?$AA@",
            b"??_C@_01FPGAMHCO@?$KH?$AA@",
            b"??_C@_01EGHLPGGP@?$KG?$AA@",
            b"??_C@_01GNFGKFKM@?$KF?$AA@",
            b"??_C@_01HEENJEON@?$KE?$AA@",
            b"??_C@_01DLAMACCK@?$KD?$AA@",
            b"??_C@_01CCBHDDGL@?$KC?$AA@",
            b"??_C@_01JDKGAKI@?$KB?$AA@",
            b"??_C@_01BACBFBOJ@?$KA?$AA@",
            b"??_C@_01EIPPHLNF@?$JP?$AA@",
            b"??_C@_01FBOEEKJE@?$JO?$AA@",
            b"??_C@_01HKMJBJFH@?$JN?$AA@",
            b"??_C@_01GDNCCIBG@?$JM?$AA@",
            b"??_C@_01CMJDLONB@?$JL?$AA@",
            b"??_C@_01DFIIIPJA@?$JK?$AA@",
            b"??_C@_01BOKFNMFD@?$JJ?$AA@",
            b"??_C@_01HLOONBC@?$JI?$AA@",
            b"??_C@_01IACGPBNN@?$JH?$AA@",
            b"??_C@_01JJDNMAJM@?$JG?$AA@",
            b"??_C@_01LCBAJDFP@?$JF?$AA@",
            b"??_C@_01KLALKCBO@?$JE?$AA@",
            b"??_C@_01OEEKDENJ@?$JD?$AA@",
            b"??_C@_01PNFBAFJI@?$JC?$AA@",
            b"??_C@_01NGHMFGFL@?$JB?$AA@",
            b"??_C@_01MPGHGHBK@?$JA?$AA@",
            b"??_C@_01CDNGJIE@?$IP?$AA@",
            b"??_C@_01BLCGFIMF@?$IO?$AA@",
            b"??_C@_01DAALALAG@?$IN?$AA@",
            b"??_C@_01CJBADKEH@?$IM?$AA@",
            b"??_C@_01GGFBKMIA@?$IL?$AA@",
            b"??_C@_01HPEKJNMB@?$IK?$AA@",
            b"??_C@_01FEGHMOAC@?$IJ?$AA@",
            b"??_C@_01ENHMPPED@?$II?$AA@",
            b"??_C@_01MKOEODIM@?$IH?$AA@",
            b"??_C@_01NDPPNCMN@?$IG?$AA@",
            b"??_C@_01PINCIBAO@?$IF?$AA@",
            b"??_C@_01OBMJLAEP@?$IE?$AA@",
            b"??_C@_01KOIICGII@?$ID?$AA@",
            b"??_C@_01LHJDBHMJ@?$IC?$AA@",
            b"??_C@_01JMLOEEAK@?$IB?$AA@",
            b"??_C@_01IFKFHFEL@?$IA?$AA@",
            b"??_C@_01BGIBIIDJ@?$HP?$AA@",
            b"??_C@_01PJKLJHI@?$HO?$AA@",
            b"??_C@_01CELHOKLL@?$HN?$AA@",
            b"??_C@_01DNKMNLPK@?$HM?$AA@",
            b"??_C@_01HCONENDN@?$HL?$AA@",
            b"??_C@_01GLPGHMHM@z?$AA@",
            b"??_C@_01EANLCPLP@y?$AA@",
            b"??_C@_01FJMABOPO@x?$AA@",
            b"??_C@_01NOFIACDB@w?$AA@",
            b"??_C@_01MHEDDDHA@v?$AA@",
            b"??_C@_01OMGOGALD@u?$AA@",
            b"??_C@_01PFHFFBPC@t?$AA@",
            b"??_C@_01LKDEMHDF@s?$AA@",
            b"??_C@_01KDCPPGHE@r?$AA@",
            b"??_C@_01IIACKFLH@q?$AA@",
            b"??_C@_01JBBJJEPG@p?$AA@",
            b"??_C@_01FMEDJKGI@o?$AA@",
            b"??_C@_01EFFIKLCJ@n?$AA@",
            b"??_C@_01GOHFPIOK@m?$AA@",
            b"??_C@_01HHGOMJKL@l?$AA@",
            b"??_C@_01DICPFPGM@k?$AA@",
            b"??_C@_01CBDEGOCN@j?$AA@",
            b"??_C@_01KBJDNOO@i?$AA@",
            b"??_C@_01BDACAMKP@h?$AA@",
            b"??_C@_01JEJKBAGA@g?$AA@",
            b"??_C@_01INIBCBCB@f?$AA@",
            b"??_C@_01KGKMHCOC@e?$AA@",
            b"??_C@_01LPLHEDKD@d?$AA@",
            b"??_C@_01PAPGNFGE@c?$AA@",
            b"??_C@_01OJONOECF@b?$AA@",
            b"??_C@_01MCMALHOG@a?$AA@",
            b"??_C@_01NLNLIGKH@?$GA?$AA@",
            b"??_C@_01IDAFKMJL@_?$AA@",
            b"??_C@_01JKBOJNNK@?$FO?$AA@",
            b"??_C@_01LBDDMOBJ@?$FN?$AA@",
            b"??_C@_01KICIPPFI@?2?$AA@",
            b"??_C@_01OHGJGJJP@?$FL?$AA@",
            b"??_C@_01POHCFINO@Z?$AA@",
            b"??_C@_01NFFPALBN@Y?$AA@",
            b"??_C@_01MMEEDKFM@X?$AA@",
            b"??_C@_01ELNMCGJD@W?$AA@",
            b"??_C@_01FCMHBHNC@V?$AA@",
            b"??_C@_01HJOKEEBB@U?$AA@",
            b"??_C@_01GAPBHFFA@T?$AA@",
            b"??_C@_01CPLAODJH@S?$AA@",
            b"??_C@_01DGKLNCNG@R?$AA@",
            b"??_C@_01BNIGIBBF@Q?$AA@",
            b"??_C@_01EJNLAFE@P?$AA@",
            b"??_C@_01MJMHLOMK@O?$AA@",
            b"??_C@_01NANMIPIL@N?$AA@",
            b"??_C@_01PLPBNMEI@M?$AA@",
            b"??_C@_01OCOKONAJ@L?$AA@",
            b"??_C@_01KNKLHLMO@K?$AA@",
            b"??_C@_01LELAEKIP@J?$AA@",
            b"??_C@_01JPJNBJEM@I?$AA@",
            b"??_C@_01IGIGCIAN@H?$AA@",
            b"??_C@_01BBODEMC@G?$AA@",
            b"??_C@_01BIAFAFID@F?$AA@",
            b"??_C@_01DDCIFGEA@E?$AA@",
            b"??_C@_01CKDDGHAB@D?$AA@",
            b"??_C@_01GFHCPBMG@C?$AA@",
            b"??_C@_01HMGJMAIH@B?$AA@",
            b"??_C@_01FHEEJDEE@A?$AA@",
            b"??_C@_01EOFPKCAF@?$EA?$AA@",
            b"??_C@_01OGPIMHDM@?$DP?$AA@",
            b"??_C@_01PPODPGHN@?$DO?$AA@",
            b"??_C@_01NEMOKFLO@?$DN?$AA@",
            b"??_C@_01MNNFJEPP@?$DM?$AA@",
            b"??_C@_01ICJEACDI@?$DL?$AA@",
            b"??_C@_01JLIPDDHJ@?3?$AA@",
            b"??_C@_01LAKCGALK@9?$AA@",
            b"??_C@_01KJLJFBPL@8?$AA@",
            b"??_C@_01COCBENDE@7?$AA@",
            b"??_C@_01DHDKHMHF@6?$AA@",
            b"??_C@_01BMBHCPLG@5?$AA@",
            b"??_C@_01FAMBOPH@4?$AA@",
            b"??_C@_01EKENIIDA@3?$AA@",
            b"??_C@_01FDFGLJHB@2?$AA@",
            b"??_C@_01HIHLOKLC@1?$AA@",
            b"??_C@_01GBGANLPD@0?$AA@",
            b"??_C@_01KMDKNFGN@?1?$AA@",
            b"??_C@_01LFCBOECM@?4?$AA@",
            b"??_C@_01JOAMLHOP@?9?$AA@",
            b"??_C@_01IHBHIGKO@?0?$AA@",
            b"??_C@_01MIFGBAGJ@?$CL?$AA@",
            b"??_C@_01NBENCBCI@?$CK?$AA@",
            b"??_C@_01PKGAHCOL@?$CJ?$AA@",
            b"??_C@_01ODHLEDKK@?$CI?$AA@",
            b"??_C@_01GEODFPGF@?8?$AA@",
            b"??_C@_01HNPIGOCE@?$CG?$AA@",
            b"??_C@_01FGNFDNOH@?$CF?$AA@",
            b"??_C@_01EPMOAMKG@$?$AA@",
            b"??_C@_01IPJKGB@?$CD?$AA@",
            b"??_C@_01BJJEKLCA@?$CC?$AA@",
            b"??_C@_01DCLJPIOD@?$CB?$AA@",
            b"??_C@_01CLKCMJKC@?5?$AA@",
            b"??_C@_01HDHMODJO@?$BP?$AA@",
            b"??_C@_01GKGHNCNP@?$BO?$AA@",
            b"??_C@_01EBEKIBBM@?$BN?$AA@",
            b"??_C@_01FIFBLAFN@?$BM?$AA@",
            b"??_C@_01BHBACGJK@?$BL?$AA@",
            b"??_C@_01OALBHNL@?$BK?$AA@",
            b"??_C@_01CFCGEEBI@?$BJ?$AA@",
            b"??_C@_01DMDNHFFJ@?$BI?$AA@",
            b"??_C@_01LLKFGJJG@?$BH?$AA@",
            b"??_C@_01KCLOFINH@?$BG?$AA@",
            b"??_C@_01IJJDALBE@?$BF?$AA@",
            b"??_C@_01JAIIDKFF@?$BE?$AA@",
            b"??_C@_01NPMJKMJC@?$BD?$AA@",
            b"??_C@_01MGNCJNND@?$BC?$AA@",
            b"??_C@_01ONPPMOBA@?$BB?$AA@",
            b"??_C@_01PEOEPPFB@?$BA?$AA@",
            b"??_C@_01DJLOPBMP@?$AP?$AA@",
            b"??_C@_01CAKFMAIO@?$AO?$AA@",
            b"??_C@_01LIIJDEN@?$AN?$AA@",
            b"??_C@_01BCJDKCAM@?$AM?$AA@",
            b"??_C@_01FNNCDEML@?$AL?$AA@",
            b"??_C@_01EEMJAFIK@?6?$AA@",
            b"??_C@_01GPOEFGEJ@?7?$AA@",
            b"??_C@_01HGPPGHAI@?$AI?$AA@",
            b"??_C@_01PBGHHLMH@?$AH?$AA@",
            b"??_C@_01OIHMEKIG@?$AG?$AA@",
            b"??_C@_01MDFBBJEF@?$AF?$AA@",
            b"??_C@_01NKEKCIAE@?$AE?$AA@",
            b"??_C@_01JFALLOMD@?$AD?$AA@",
            b"??_C@_01IMBAIPIC@?$AC?$AA@",
            b"??_C@_01KHDNNMEB@?$AB?$AA@",
            b"??_C@_01LOCGONAA@?$AA?$AA@",
        ];
        let outputs: [&[u8]; 256] = [
            b"\"\\xFF\"",
            b"\"\\xFE\"",
            b"\"\\xFD\"",
            b"\"\\xFC\"",
            b"\"\\xFB\"",
            b"\"\\xFA\"",
            b"\"\\xF9\"",
            b"\"\\xF8\"",
            b"\"\\xF7\"",
            b"\"\\xF6\"",
            b"\"\\xF5\"",
            b"\"\\xF4\"",
            b"\"\\xF3\"",
            b"\"\\xF2\"",
            b"\"\\xF1\"",
            b"\"\\xF0\"",
            b"\"\\xEF\"",
            b"\"\\xEE\"",
            b"\"\\xED\"",
            b"\"\\xEC\"",
            b"\"\\xEB\"",
            b"\"\\xEA\"",
            b"\"\\xE9\"",
            b"\"\\xE8\"",
            b"\"\\xE7\"",
            b"\"\\xE6\"",
            b"\"\\xE5\"",
            b"\"\\xE4\"",
            b"\"\\xE3\"",
            b"\"\\xE2\"",
            b"\"\\xE1\"",
            b"\"\\xE0\"",
            b"\"\\xDF\"",
            b"\"\\xDE\"",
            b"\"\\xDD\"",
            b"\"\\xDC\"",
            b"\"\\xDB\"",
            b"\"\\xDA\"",
            b"\"\\xD9\"",
            b"\"\\xD8\"",
            b"\"\\xD7\"",
            b"\"\\xD6\"",
            b"\"\\xD5\"",
            b"\"\\xD4\"",
            b"\"\\xD3\"",
            b"\"\\xD2\"",
            b"\"\\xD1\"",
            b"\"\\xD0\"",
            b"\"\\xCF\"",
            b"\"\\xCE\"",
            b"\"\\xCD\"",
            b"\"\\xCC\"",
            b"\"\\xCB\"",
            b"\"\\xCA\"",
            b"\"\\xC9\"",
            b"\"\\xC8\"",
            b"\"\\xC7\"",
            b"\"\\xC6\"",
            b"\"\\xC5\"",
            b"\"\\xC4\"",
            b"\"\\xC3\"",
            b"\"\\xC2\"",
            b"\"\\xC1\"",
            b"\"\\xC0\"",
            b"\"\\xBF\"",
            b"\"\\xBE\"",
            b"\"\\xBD\"",
            b"\"\\xBC\"",
            b"\"\\xBB\"",
            b"\"\\xBA\"",
            b"\"\\xB9\"",
            b"\"\\xB8\"",
            b"\"\\xB7\"",
            b"\"\\xB6\"",
            b"\"\\xB5\"",
            b"\"\\xB4\"",
            b"\"\\xB3\"",
            b"\"\\xB2\"",
            b"\"\\xB1\"",
            b"\"\\xB0\"",
            b"\"\\xAF\"",
            b"\"\\xAE\"",
            b"\"\\xAD\"",
            b"\"\\xAC\"",
            b"\"\\xAB\"",
            b"\"\\xAA\"",
            b"\"\\xA9\"",
            b"\"\\xA8\"",
            b"\"\\xA7\"",
            b"\"\\xA6\"",
            b"\"\\xA5\"",
            b"\"\\xA4\"",
            b"\"\\xA3\"",
            b"\"\\xA2\"",
            b"\"\\xA1\"",
            b"\"\\xA0\"",
            b"\"\\x9F\"",
            b"\"\\x9E\"",
            b"\"\\x9D\"",
            b"\"\\x9C\"",
            b"\"\\x9B\"",
            b"\"\\x9A\"",
            b"\"\\x99\"",
            b"\"\\x98\"",
            b"\"\\x97\"",
            b"\"\\x96\"",
            b"\"\\x95\"",
            b"\"\\x94\"",
            b"\"\\x93\"",
            b"\"\\x92\"",
            b"\"\\x91\"",
            b"\"\\x90\"",
            b"\"\\x8F\"",
            b"\"\\x8E\"",
            b"\"\\x8D\"",
            b"\"\\x8C\"",
            b"\"\\x8B\"",
            b"\"\\x8A\"",
            b"\"\\x89\"",
            b"\"\\x88\"",
            b"\"\\x87\"",
            b"\"\\x86\"",
            b"\"\\x85\"",
            b"\"\\x84\"",
            b"\"\\x83\"",
            b"\"\\x82\"",
            b"\"\\x81\"",
            b"\"\\x80\"",
            b"\"\\x7F\"",
            b"\"~\"",
            b"\"}\"",
            b"\"|\"",
            b"\"{\"",
            b"\"z\"",
            b"\"y\"",
            b"\"x\"",
            b"\"w\"",
            b"\"v\"",
            b"\"u\"",
            b"\"t\"",
            b"\"s\"",
            b"\"r\"",
            b"\"q\"",
            b"\"p\"",
            b"\"o\"",
            b"\"n\"",
            b"\"m\"",
            b"\"l\"",
            b"\"k\"",
            b"\"j\"",
            b"\"i\"",
            b"\"h\"",
            b"\"g\"",
            b"\"f\"",
            b"\"e\"",
            b"\"d\"",
            b"\"c\"",
            b"\"b\"",
            b"\"a\"",
            b"\"`\"",
            b"\"_\"",
            b"\"^\"",
            b"\"]\"",
            b"\"\\\\\"",
            b"\"[\"",
            b"\"Z\"",
            b"\"Y\"",
            b"\"X\"",
            b"\"W\"",
            b"\"V\"",
            b"\"U\"",
            b"\"T\"",
            b"\"S\"",
            b"\"R\"",
            b"\"Q\"",
            b"\"P\"",
            b"\"O\"",
            b"\"N\"",
            b"\"M\"",
            b"\"L\"",
            b"\"K\"",
            b"\"J\"",
            b"\"I\"",
            b"\"H\"",
            b"\"G\"",
            b"\"F\"",
            b"\"E\"",
            b"\"D\"",
            b"\"C\"",
            b"\"B\"",
            b"\"A\"",
            b"\"@\"",
            b"\"?\"",
            b"\">\"",
            b"\"=\"",
            b"\"<\"",
            b"\";\"",
            b"\":\"",
            b"\"9\"",
            b"\"8\"",
            b"\"7\"",
            b"\"6\"",
            b"\"5\"",
            b"\"4\"",
            b"\"3\"",
            b"\"2\"",
            b"\"1\"",
            b"\"0\"",
            b"\"/\"",
            b"\".\"",
            b"\"-\"",
            b"\",\"",
            b"\"+\"",
            b"\"*\"",
            b"\")\"",
            b"\"(\"",
            b"\"\\'\"",
            b"\"&\"",
            b"\"%\"",
            b"\"$\"",
            b"\"#\"",
            b"\"\\\"\"",
            b"\"!\"",
            b"\" \"",
            b"\"\\x1F\"",
            b"\"\\x1E\"",
            b"\"\\x1D\"",
            b"\"\\x1C\"",
            b"\"\\x1B\"",
            b"\"\\x1A\"",
            b"\"\\x19\"",
            b"\"\\x18\"",
            b"\"\\x17\"",
            b"\"\\x16\"",
            b"\"\\x15\"",
            b"\"\\x14\"",
            b"\"\\x13\"",
            b"\"\\x12\"",
            b"\"\\x11\"",
            b"\"\\x10\"",
            b"\"\\x0F\"",
            b"\"\\x0E\"",
            b"\"\\r\"",
            b"\"\\f\"",
            b"\"\\v\"",
            b"\"\\n\"",
            b"\"\\t\"",
            b"\"\\b\"",
            b"\"\\a\"",
            b"\"\\x06\"",
            b"\"\\x05\"",
            b"\"\\x04\"",
            b"\"\\x03\"",
            b"\"\\x02\"",
            b"\"\\x01\"",
            b"u\"\"",
        ];

        for (input, output) in inputs.iter().zip(outputs) {
            test(input, output);
        }
    }

    {
        let inputs: [&[u8]; 98] = [
            b"??_C@_13KDLDGPGJ@?$AA?7?$AA?$AA@",
            b"??_C@_13LBAGMAIH@?$AA?6?$AA?$AA@",
            b"??_C@_13JLKKHOC@?$AA?$AL?$AA?$AA@",
            b"??_C@_13HOIJIPNN@?$AA?5?$AA?$AA@",
            b"??_C@_13MGDFOILI@?$AA?$CB?$AA?$AA@",
            b"??_C@_13NEIAEHFG@?$AA?$CC?$AA?$AA@",
            b"??_C@_13GMDMCADD@?$AA?$CD?$AA?$AA@",
            b"??_C@_13PBOLBIIK@?$AA$?$AA?$AA@",
            b"??_C@_13EJFHHPOP@?$AA?$CF?$AA?$AA@",
            b"??_C@_13FLOCNAAB@?$AA?$CG?$AA?$AA@",
            b"??_C@_13ODFOLHGE@?$AA?8?$AA?$AA@",
            b"??_C@_13LLDNKHDC@?$AA?$CI?$AA?$AA@",
            b"??_C@_13DIBMAFH@?$AA?$CJ?$AA?$AA@",
            b"??_C@_13BBDEGPLJ@?$AA?$CK?$AA?$AA@",
            b"??_C@_13KJIIAINM@?$AA?$CL?$AA?$AA@",
            b"??_C@_13DEFPDAGF@?$AA?0?$AA?$AA@",
            b"??_C@_13IMODFHAA@?$AA?9?$AA?$AA@",
            b"??_C@_13JOFGPIOO@?$AA?4?$AA?$AA@",
            b"??_C@_13CGOKJPIL@?$AA?1?$AA?$AA@",
            b"??_C@_13COJANIEC@?$AA0?$AA?$AA@",
            b"??_C@_13JGCMLPCH@?$AA1?$AA?$AA@",
            b"??_C@_13IEJJBAMJ@?$AA2?$AA?$AA@",
            b"??_C@_13DMCFHHKM@?$AA3?$AA?$AA@",
            b"??_C@_13KBPCEPBF@?$AA4?$AA?$AA@",
            b"??_C@_13BJEOCIHA@?$AA5?$AA?$AA@",
            b"??_C@_13LPLIHJO@?$AA6?$AA?$AA@",
            b"??_C@_13LDEHOAPL@?$AA7?$AA?$AA@",
            b"??_C@_13OLCEPAKN@?$AA8?$AA?$AA@",
            b"??_C@_13FDJIJHMI@?$AA9?$AA?$AA@",
            b"??_C@_13EBCNDICG@?$AA?3?$AA?$AA@",
            b"??_C@_13PJJBFPED@?$AA?$DL?$AA?$AA@",
            b"??_C@_13GEEGGHPK@?$AA?$DM?$AA?$AA@",
            b"??_C@_13NMPKAAJP@?$AA?$DN?$AA?$AA@",
            b"??_C@_13MOEPKPHB@?$AA?$DO?$AA?$AA@",
            b"??_C@_13HGPDMIBE@?$AA?$DP?$AA?$AA@",
            b"??_C@_13EFKPHINO@?$AA?$EA?$AA?$AA@",
            b"??_C@_13PNBDBPLL@?$AAA?$AA?$AA@",
            b"??_C@_13OPKGLAFF@?$AAB?$AA?$AA@",
            b"??_C@_13FHBKNHDA@?$AAC?$AA?$AA@",
            b"??_C@_13MKMNOPIJ@?$AAD?$AA?$AA@",
            b"??_C@_13HCHBIIOM@?$AAE?$AA?$AA@",
            b"??_C@_13GAMECHAC@?$AAF?$AA?$AA@",
            b"??_C@_13NIHIEAGH@?$AAG?$AA?$AA@",
            b"??_C@_13IABLFADB@?$AAH?$AA?$AA@",
            b"??_C@_13DIKHDHFE@?$AAI?$AA?$AA@",
            b"??_C@_13CKBCJILK@?$AAJ?$AA?$AA@",
            b"??_C@_13JCKOPPNP@?$AAK?$AA?$AA@",
            b"??_C@_13PHJMHGG@?$AAL?$AA?$AA@",
            b"??_C@_13LHMFKAAD@?$AAM?$AA?$AA@",
            b"??_C@_13KFHAAPON@?$AAN?$AA?$AA@",
            b"??_C@_13BNMMGIII@?$AAO?$AA?$AA@",
            b"??_C@_13BFLGCPEB@?$AAP?$AA?$AA@",
            b"??_C@_13KNAKEICE@?$AAQ?$AA?$AA@",
            b"??_C@_13LPLPOHMK@?$AAR?$AA?$AA@",
            b"??_C@_13HADIAKP@?$AAS?$AA?$AA@",
            b"??_C@_13JKNELIBG@?$AAT?$AA?$AA@",
            b"??_C@_13CCGINPHD@?$AAU?$AA?$AA@",
            b"??_C@_13DANNHAJN@?$AAV?$AA?$AA@",
            b"??_C@_13IIGBBHPI@?$AAW?$AA?$AA@",
            b"??_C@_13NAACAHKO@?$AAX?$AA?$AA@",
            b"??_C@_13GILOGAML@?$AAY?$AA?$AA@",
            b"??_C@_13HKALMPCF@?$AAZ?$AA?$AA@",
            b"??_C@_13MCLHKIEA@?$AA?$FL?$AA?$AA@",
            b"??_C@_13FPGAJAPJ@?$AA?2?$AA?$AA@",
            b"??_C@_13OHNMPHJM@?$AA?$FN?$AA?$AA@",
            b"??_C@_13PFGJFIHC@?$AA?$FO?$AA?$AA@",
            b"??_C@_13ENNFDPBH@?$AA_?$AA?$AA@",
            b"??_C@_13OFJNNHOA@?$AA?$GA?$AA?$AA@",
            b"??_C@_13FNCBLAIF@?$AAa?$AA?$AA@",
            b"??_C@_13EPJEBPGL@?$AAb?$AA?$AA@",
            b"??_C@_13PHCIHIAO@?$AAc?$AA?$AA@",
            b"??_C@_13GKPPEALH@?$AAd?$AA?$AA@",
            b"??_C@_13NCEDCHNC@?$AAe?$AA?$AA@",
            b"??_C@_13MAPGIIDM@?$AAf?$AA?$AA@",
            b"??_C@_13HIEKOPFJ@?$AAg?$AA?$AA@",
            b"??_C@_13CACJPPAP@?$AAh?$AA?$AA@",
            b"??_C@_13JIJFJIGK@?$AAi?$AA?$AA@",
            b"??_C@_13IKCADHIE@?$AAj?$AA?$AA@",
            b"??_C@_13DCJMFAOB@?$AAk?$AA?$AA@",
            b"??_C@_13KPELGIFI@?$AAl?$AA?$AA@",
            b"??_C@_13BHPHAPDN@?$AAm?$AA?$AA@",
            b"??_C@_13FECKAND@?$AAn?$AA?$AA@",
            b"??_C@_13LNPOMHLG@?$AAo?$AA?$AA@",
            b"??_C@_13LFIEIAHP@?$AAp?$AA?$AA@",
            b"??_C@_13NDIOHBK@?$AAq?$AA?$AA@",
            b"??_C@_13BPINEIPE@?$AAr?$AA?$AA@",
            b"??_C@_13KHDBCPJB@?$AAs?$AA?$AA@",
            b"??_C@_13DKOGBHCI@?$AAt?$AA?$AA@",
            b"??_C@_13ICFKHAEN@?$AAu?$AA?$AA@",
            b"??_C@_13JAOPNPKD@?$AAv?$AA?$AA@",
            b"??_C@_13CIFDLIMG@?$AAw?$AA?$AA@",
            b"??_C@_13HADAKIJA@?$AAx?$AA?$AA@",
            b"??_C@_13MIIMMPPF@?$AAy?$AA?$AA@",
            b"??_C@_13NKDJGABL@?$AAz?$AA?$AA@",
            b"??_C@_13GCIFAHHO@?$AA?$HL?$AA?$AA@",
            b"??_C@_13PPFCDPMH@?$AA?$HM?$AA?$AA@",
            b"??_C@_13EHOOFIKC@?$AA?$HN?$AA?$AA@",
            b"??_C@_13FFFLPHEM@?$AA?$HO?$AA?$AA@",
        ];
        let outputs: [&[u8]; 98] = [
            b"\"\\t\"",
            b"\"\\n\"",
            b"\"\\v\"",
            b"\" \"",
            b"\"!\"",
            b"\"\\\"\"",
            b"\"#\"",
            b"\"$\"",
            b"\"%\"",
            b"\"&\"",
            b"\"\\'\"",
            b"\"(\"",
            b"\")\"",
            b"\"*\"",
            b"\"+\"",
            b"\",\"",
            b"\"-\"",
            b"\".\"",
            b"\"/\"",
            b"\"0\"",
            b"\"1\"",
            b"\"2\"",
            b"\"3\"",
            b"\"4\"",
            b"\"5\"",
            b"\"6\"",
            b"\"7\"",
            b"\"8\"",
            b"\"9\"",
            b"\":\"",
            b"\";\"",
            b"\"<\"",
            b"\"=\"",
            b"\">\"",
            b"\"?\"",
            b"\"@\"",
            b"\"A\"",
            b"\"B\"",
            b"\"C\"",
            b"\"D\"",
            b"\"E\"",
            b"\"F\"",
            b"\"G\"",
            b"\"H\"",
            b"\"I\"",
            b"\"J\"",
            b"\"K\"",
            b"\"L\"",
            b"\"M\"",
            b"\"N\"",
            b"\"O\"",
            b"\"P\"",
            b"\"Q\"",
            b"\"R\"",
            b"\"S\"",
            b"\"T\"",
            b"\"U\"",
            b"\"V\"",
            b"\"W\"",
            b"\"X\"",
            b"\"Y\"",
            b"\"Z\"",
            b"\"[\"",
            b"\"\\\\\"",
            b"\"]\"",
            b"\"^\"",
            b"\"_\"",
            b"\"`\"",
            b"\"a\"",
            b"\"b\"",
            b"\"c\"",
            b"\"d\"",
            b"\"e\"",
            b"\"f\"",
            b"\"g\"",
            b"\"h\"",
            b"\"i\"",
            b"\"j\"",
            b"\"k\"",
            b"\"l\"",
            b"\"m\"",
            b"\"n\"",
            b"\"o\"",
            b"\"p\"",
            b"\"q\"",
            b"\"r\"",
            b"\"s\"",
            b"\"t\"",
            b"\"u\"",
            b"\"v\"",
            b"\"w\"",
            b"\"x\"",
            b"\"y\"",
            b"\"z\"",
            b"\"{\"",
            b"\"|\"",
            b"\"}\"",
            b"\"~\"",
        ];

        for (input, output) in inputs.iter().zip(outputs) {
            test(input, output);
        }
    }

    test(
        b"??_C@_0CF@LABBIIMO@012345678901234567890123456789AB@",
        b"\"012345678901234567890123456789AB\"...",
    );
    test(b"??_C@_1EK@KFPEBLPK@?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AAA?$AAB@", b"L\"012345678901234567890123456789AB\"...");
    test(b"??_C@_13IIHIAFKH@?W?$PP?$AA?$AA@", b"L\"\\xD7FF\"");
    test(b"??_C@_03IIHIAFKH@?$PP?W?$AA?$AA@", b"u\"\\xD7FF\"");
    test(b"??_C@_02PCEFGMJL@hi?$AA@", b"\"hi\"");
    test(b"??_C@_05OMLEGLOC@h?$AAi?$AA?$AA?$AA@", b"u\"hi\"");
    test(b"??_C@_0EK@FEAOBHPP@o?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA@", b"u\"o123456789012345\"...");
    test(
        b"??_C@_0M@GFNAJIPG@h?$AA?$AA?$AAi?$AA?$AA?$AA?$AA?$AA?$AA?$AA@",
        b"U\"hi\"",
    );
    test(b"??_C@_0JE@IMHFEDAA@0?$AA?$AA?$AA1?$AA?$AA?$AA2?$AA?$AA?$AA3?$AA?$AA?$AA4?$AA?$AA?$AA5?$AA?$AA?$AA6?$AA?$AA?$AA7?$AA?$AA?$AA@", b"U\"01234567\"...");
    test(
        b"??_C@_0CA@NMANGEKF@012345678901234567890123456789A?$AA@",
        b"\"012345678901234567890123456789A\"",
    );
    test(b"??_C@_1EA@LJAFPILO@?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AAA?$AA?$AA@", b"L\"012345678901234567890123456789A\"");
    test(
        b"??_C@_0CA@NMANGEKF@012345678901234567890123456789A?$AA@",
        b"\"012345678901234567890123456789A\"",
    );
    test(b"??_C@_0CA@NFEFHIFO@0?$AA1?$AA2?$AA3?$AA4?$AA5?$AA6?$AA7?$AA8?$AA9?$AA0?$AA1?$AA2?$AA3?$AA4?$AA?$AA?$AA@", b"u\"012345678901234\"");
    test(b"??_C@_0CA@KFPHPCC@0?$AA?$AA?$AA1?$AA?$AA?$AA2?$AA?$AA?$AA3?$AA?$AA?$AA4?$AA?$AA?$AA5?$AA?$AA?$AA6?$AA?$AA?$AA?$AA?$AA?$AA?$AA@", b"U\"0123456\"");
    test(b"??_C@_0CG@HJGBPLNO@l?$AAo?$AAo?$AAk?$AAA?$AAh?$AAe?$AAa?$AAd?$AAH?$AAa?$AAr?$AAd?$AAB?$AAr?$AAe?$AAa?$AAk?$AA?$AA?$AA@", b"u\"lookAheadHardBreak\"");
    test(b"??_C@_0CG@HJGBPLNO@l?$AAo?$AAo?$AAk?$AAA?$AAh?$AAe?$AAa?$AAd?$AAH?$AAa?$AAr?$AAd?$AAB?$AAr?$AAe?$AA@", b"u\"lookAheadHardBre\"...");
    test(b"??_C@_05LABPAAN@b?$AA?$AA?$AA?$AA?$AA@", b"u\"b\\0\"");
    test(b"??_C@_0CC@MBPKDIAM@a?$AA?$AA?$AAb?$AA?$AA?$AAc?$AA?$AA?$AAd?$AA?$AA?$AAe?$AA?$AA?$AAf?$AA?$AA?$AAg?$AA?$AA?$AAh?$AA?$AA?$AA@", b"u\"a\\0b\\0c\\0d\\0e\\0f\\0g\\0h\\0\"...");
    test(
        b"??_C@_07LJGFEJEB@D3?$CC?$BB?$AA?$AA?$AA?$AA@)",
        b"U\"\\x11223344\"",
    );
    test(
        b"??_C@_0GAAAAAAAA@GPLEPFHO@01234567890123456789012345678901@",
        b"\"01234567890123456789012345678901\"...",
    );
}

#[test]
fn test_template_callback() {
    test(
        b"?callback_void@@3V?$C@$$A6AXXZ@@A",
        b"class C<void __cdecl(void)> callback_void",
    );
    test(
        b"?callback_void_volatile@@3V?$C@$$A6AXXZ@@C",
        b"class C<void __cdecl(void)> volatile callback_void_volatile",
    );
    test(
        b"?callback_int@@3V?$C@$$A6AHXZ@@A",
        b"C<int __cdecl(void)> callback_int",
    );
    test(
        b"?callback_Type@@3V?$C@$$A6A?AVType@@XZ@@A",
        b"C<class Type __cdecl(void)> callback_Type",
    );
    test(
        b"?callback_void_int@@3V?$C@$$A6AXH@Z@@A",
        b"C<void __cdecl(int)> callback_void_int",
    );
    test(
        b"?callback_int_int@@3V?$C@$$A6AHH@Z@@A",
        b"C<int __cdecl(int)> callback_int_int",
    );
    test(
        b"?callback_void_Type@@3V?$C@$$A6AXVType@@@Z@@A",
        b"C<void __cdecl(class Type)> callback_void_Type",
    );
    test(
        b"?foo@@YAXV?$C@$$A6AXXZ@@@Z",
        b"void __cdecl foo(class C<void __cdecl(void)>)",
    );
    test(
        b"?function@@YAXV?$C@$$A6AXXZ@@@Z",
        b"void __cdecl function(class C<void __cdecl(void)>)",
    );
    test(
        b"?function_pointer@@YAXV?$C@P6AXXZ@@@Z",
        b"void __cdecl function_pointer(class C<void (__cdecl *)(void)>)",
    );
    test(
        b"?member_pointer@@YAXV?$C@P8Z@@AEXXZ@@@Z",
        b"void __cdecl member_pointer(class C<void (__thiscall Z::*)(void)>)",
    );
    test(
        b"??$bar@P6AHH@Z@@YAXP6AHH@Z@Z",
        b"void __cdecl bar<int (__cdecl *)(int)>(int (__cdecl *)(int))",
    );
    test(
        b"??$WrapFnPtr@$1?VoidFn@@YAXXZ@@YAXXZ",
        b"void __cdecl WrapFnPtr<&void __cdecl VoidFn(void)>(void)",
    );
    test(
        b"??$WrapFnRef@$1?VoidFn@@YAXXZ@@YAXXZ",
        b"void __cdecl WrapFnRef<&void __cdecl VoidFn(void)>(void)",
    );
    test(
        b"??$WrapFnPtr@$1?VoidStaticMethod@Thing@@SAXXZ@@YAXXZ",
        b"void __cdecl WrapFnPtr<&public: static void __cdecl Thing::VoidStaticMethod(void)>(void)",
    );
    test(
        b"??$WrapFnRef@$1?VoidStaticMethod@Thing@@SAXXZ@@YAXXZ",
        b"void __cdecl WrapFnRef<&public: static void __cdecl Thing::VoidStaticMethod(void)>(void)",
    );
}

#[test]
fn test_templates_memptrs_2() {
    test(b"?m@@3U?$J@UM@@$0A@@@A", b"struct J<struct M, 0> m");
    test(b"?m2@@3U?$K@UM@@$0?0@@A", b"struct K<struct M, -1> m2");
    test(b"?n@@3U?$J@UN@@$HA@@@A", b"struct J<struct N, {0}> n");
    test(b"?n2@@3U?$K@UN@@$0?0@@A", b"struct K<struct N, -1> n2");
    test(b"?o@@3U?$J@UO@@$IA@A@@@A", b"struct J<struct O, {0, 0}> o");
    test(
        b"?o2@@3U?$K@UO@@$FA@?0@@A",
        b"struct K<struct O, {0, -1}> o2",
    );
    test(
        b"?p@@3U?$J@UP@@$JA@A@?0@@A",
        b"struct J<struct P, {0, 0, -1}> p",
    );
    test(
        b"?p2@@3U?$K@UP@@$GA@A@?0@@A",
        b"struct K<struct P, {0, 0, -1}> p2",
    );
    test(b"??0?$ClassTemplate@$J??_9MostGeneral@@$BA@AEA@M@3@@QAE@XZ", b"__thiscall ClassTemplate<{[thunk]: __thiscall MostGeneral::`vcall'{0, {flat}}, 0, 12, 4}>::ClassTemplate<{[thunk]: __thiscall MostGeneral::`vcall'{0, {flat}}, 0, 12, 4}>(void)");
}

#[test]
fn test_templates_memptrs() {
    test(b"??$CallMethod@UC@NegativeNVOffset@@$I??_912@$BA@AEPPPPPPPM@A@@@YAXAAUC@NegativeNVOffset@@@Z", b"void __cdecl CallMethod<struct NegativeNVOffset::C, {[thunk]: __thiscall NegativeNVOffset::C::`vcall'{0, {flat}}, 4294967292, 0}>(struct NegativeNVOffset::C &)");
    test(
        b"??$CallMethod@UM@@$0A@@@YAXAAUM@@@Z",
        b"void __cdecl CallMethod<struct M, 0>(struct M &)",
    );
    test(b"??$CallMethod@UM@@$H??_91@$BA@AEA@@@YAXAAUM@@@Z", b"void __cdecl CallMethod<struct M, {[thunk]: __thiscall M::`vcall'{0, {flat}}, 0}>(struct M &)");
    test(
        b"??$CallMethod@UM@@$H?f@1@QAEXXZA@@@YAXAAUM@@@Z",
        b"void __cdecl CallMethod<struct M, {public: void __thiscall M::f(void), 0}>(struct M &)",
    );
    test(b"??$CallMethod@UO@@$H??_91@$BA@AE3@@YAXAAUO@@@Z", b"void __cdecl CallMethod<struct O, {[thunk]: __thiscall O::`vcall'{0, {flat}}, 4}>(struct O &)");
    test(
        b"??$CallMethod@US@@$0A@@@YAXAAUS@@@Z",
        b"void __cdecl CallMethod<struct S, 0>(struct S &)",
    );
    test(b"??$CallMethod@US@@$1??_91@$BA@AE@@YAXAAUS@@@Z", b"void __cdecl CallMethod<struct S, &[thunk]: __thiscall S::`vcall'{0, {flat}}>(struct S &)");
    test(
        b"??$CallMethod@US@@$1?f@1@QAEXXZ@@YAXAAUS@@@Z",
        b"void __cdecl CallMethod<struct S, &public: void __thiscall S::f(void)>(struct S &)",
    );
    test(
        b"??$CallMethod@UU@@$0A@@@YAXAAUU@@@Z",
        b"void __cdecl CallMethod<struct U, 0>(struct U &)",
    );
    test(b"??$CallMethod@UU@@$J??_91@$BA@AEA@A@A@@@YAXAAUU@@@Z", b"void __cdecl CallMethod<struct U, {[thunk]: __thiscall U::`vcall'{0, {flat}}, 0, 0, 0}>(struct U &)");
    test(b"??$CallMethod@UU@@$J?f@1@QAEXXZA@A@A@@@YAXAAUU@@@Z", b"void __cdecl CallMethod<struct U, {public: void __thiscall U::f(void), 0, 0, 0}>(struct U &)");
    test(
        b"??$CallMethod@UV@@$0A@@@YAXAAUV@@@Z",
        b"void __cdecl CallMethod<struct V, 0>(struct V &)",
    );
    test(b"??$CallMethod@UV@@$I??_91@$BA@AEA@A@@@YAXAAUV@@@Z", b"void __cdecl CallMethod<struct V, {[thunk]: __thiscall V::`vcall'{0, {flat}}, 0, 0}>(struct V &)");
    test(b"??$CallMethod@UV@@$I?f@1@QAEXXZA@A@@@YAXAAUV@@@Z", b"void __cdecl CallMethod<struct V, {public: void __thiscall V::f(void), 0, 0}>(struct V &)");
    test(
        b"??$ReadField@UA@@$0?0@@YAHAAUA@@@Z",
        b"int __cdecl ReadField<struct A, -1>(struct A &)",
    );
    test(
        b"??$ReadField@UA@@$0A@@@YAHAAUA@@@Z",
        b"int __cdecl ReadField<struct A, 0>(struct A &)",
    );
    test(
        b"??$ReadField@UI@@$03@@YAHAAUI@@@Z",
        b"int __cdecl ReadField<struct I, 4>(struct I &)",
    );
    test(
        b"??$ReadField@UI@@$0A@@@YAHAAUI@@@Z",
        b"int __cdecl ReadField<struct I, 0>(struct I &)",
    );
    test(
        b"??$ReadField@UM@@$0A@@@YAHAAUM@@@Z",
        b"int __cdecl ReadField<struct M, 0>(struct M &)",
    );
    test(
        b"??$ReadField@UM@@$0BA@@@YAHAAUM@@@Z",
        b"int __cdecl ReadField<struct M, 16>(struct M &)",
    );
    test(
        b"??$ReadField@UM@@$0M@@@YAHAAUM@@@Z",
        b"int __cdecl ReadField<struct M, 12>(struct M &)",
    );
    test(
        b"??$ReadField@US@@$03@@YAHAAUS@@@Z",
        b"int __cdecl ReadField<struct S, 4>(struct S &)",
    );
    test(
        b"??$ReadField@US@@$07@@YAHAAUS@@@Z",
        b"int __cdecl ReadField<struct S, 8>(struct S &)",
    );
    test(
        b"??$ReadField@US@@$0A@@@YAHAAUS@@@Z",
        b"int __cdecl ReadField<struct S, 0>(struct S &)",
    );
    test(
        b"??$ReadField@UU@@$0A@@@YAHAAUU@@@Z",
        b"int __cdecl ReadField<struct U, 0>(struct U &)",
    );
    test(
        b"??$ReadField@UU@@$G3A@A@@@YAHAAUU@@@Z",
        b"int __cdecl ReadField<struct U, {4, 0, 0}>(struct U &)",
    );
    test(
        b"??$ReadField@UU@@$G7A@A@@@YAHAAUU@@@Z",
        b"int __cdecl ReadField<struct U, {8, 0, 0}>(struct U &)",
    );
    test(
        b"??$ReadField@UV@@$0A@@@YAHAAUV@@@Z",
        b"int __cdecl ReadField<struct V, 0>(struct V &)",
    );
    test(
        b"??$ReadField@UV@@$F7A@@@YAHAAUV@@@Z",
        b"int __cdecl ReadField<struct V, {8, 0}>(struct V &)",
    );
    test(
        b"??$ReadField@UV@@$FM@A@@@YAHAAUV@@@Z",
        b"int __cdecl ReadField<struct V, {12, 0}>(struct V &)",
    );
    test(
        b"?Q@@3$$QEAP8Foo@@EAAXXZEA",
        b"void (__cdecl Foo::*&&Q)(void)",
    );
}

#[test]
fn test_templates() {
    test(b"?f@@3V?$C@H@@A", b"class C<int> f");
    test(
        b"??0?$Class@VTypename@@@@QAE@XZ",
        b"__thiscall Class<class Typename>::Class<class Typename>(void)",
    );
    test(
        b"??0?$Class@VTypename@@@@QEAA@XZ",
        b"__cdecl Class<class Typename>::Class<class Typename>(void)",
    );
    test(
        b"??0?$Class@$$CBVTypename@@@@QAE@XZ",
        b"__thiscall Class<class Typename const>::Class<class Typename const>(void)",
    );
    test(
        b"??0?$Class@$$CBVTypename@@@@QEAA@XZ",
        b"__cdecl Class<class Typename const>::Class<class Typename const>(void)",
    );
    test(
        b"??0?$Class@$$CCVTypename@@@@QAE@XZ",
        b"__thiscall Class<class Typename volatile>::Class<class Typename volatile>(void)",
    );
    test(
        b"??0?$Class@$$CCVTypename@@@@QEAA@XZ",
        b"__cdecl Class<class Typename volatile>::Class<class Typename volatile>(void)",
    );
    test(b"??0?$Class@$$CDVTypename@@@@QAE@XZ", b"__thiscall Class<class Typename const volatile>::Class<class Typename const volatile>(void)");
    test(
        b"??0?$Class@$$CDVTypename@@@@QEAA@XZ",
        b"__cdecl Class<class Typename const volatile>::Class<class Typename const volatile>(void)",
    );
    test(b"??0?$Class@V?$Nested@VTypename@@@@@@QAE@XZ", b"__thiscall Class<class Nested<class Typename>>::Class<class Nested<class Typename>>(void)");
    test(
        b"??0?$Class@V?$Nested@VTypename@@@@@@QEAA@XZ",
        b"__cdecl Class<class Nested<class Typename>>::Class<class Nested<class Typename>>(void)",
    );
    test(
        b"??0?$Class@QAH@@QAE@XZ",
        b"__thiscall Class<int *const>::Class<int *const>(void)",
    );
    test(
        b"??0?$Class@QEAH@@QEAA@XZ",
        b"__cdecl Class<int *const>::Class<int *const>(void)",
    );
    test(
        b"??0?$Class@$$A6AHXZ@@QAE@XZ",
        b"__thiscall Class<int __cdecl(void)>::Class<int __cdecl(void)>(void)",
    );
    test(
        b"??0?$Class@$$A6AHXZ@@QEAA@XZ",
        b"__cdecl Class<int __cdecl(void)>::Class<int __cdecl(void)>(void)",
    );
    test(
        b"??0?$Class@$$BY0A@H@@QAE@XZ",
        b"__thiscall Class<int[]>::Class<int[]>(void)",
    );
    test(
        b"??0?$Class@$$BY0A@H@@QEAA@XZ",
        b"__cdecl Class<int[]>::Class<int[]>(void)",
    );
    test(
        b"??0?$Class@$$BY04H@@QAE@XZ",
        b"__thiscall Class<int[5]>::Class<int[5]>(void)",
    );
    test(
        b"??0?$Class@$$BY04H@@QEAA@XZ",
        b"__cdecl Class<int[5]>::Class<int[5]>(void)",
    );
    test(
        b"??0?$Class@$$BY04$$CBH@@QAE@XZ",
        b"__thiscall Class<int const[5]>::Class<int const[5]>(void)",
    );
    test(
        b"??0?$Class@$$BY04$$CBH@@QEAA@XZ",
        b"__cdecl Class<int const[5]>::Class<int const[5]>(void)",
    );
    test(
        b"??0?$Class@$$BY04QAH@@QAE@XZ",
        b"__thiscall Class<int *const[5]>::Class<int *const[5]>(void)",
    );
    test(
        b"??0?$Class@$$BY04QEAH@@QEAA@XZ",
        b"__cdecl Class<int *const[5]>::Class<int *const[5]>(void)",
    );
    test(
        b"??0?$BoolTemplate@$0A@@@QAE@XZ",
        b"__thiscall BoolTemplate<0>::BoolTemplate<0>(void)",
    );
    test(
        b"??0?$BoolTemplate@$0A@@@QEAA@XZ",
        b"__cdecl BoolTemplate<0>::BoolTemplate<0>(void)",
    );
    test(
        b"??0?$BoolTemplate@$00@@QAE@XZ",
        b"__thiscall BoolTemplate<1>::BoolTemplate<1>(void)",
    );
    test(
        b"??0?$BoolTemplate@$00@@QEAA@XZ",
        b"__cdecl BoolTemplate<1>::BoolTemplate<1>(void)",
    );
    test(
        b"??$Foo@H@?$BoolTemplate@$00@@QAEXH@Z",
        b"void __thiscall BoolTemplate<1>::Foo<int>(int)",
    );
    test(
        b"??$Foo@H@?$BoolTemplate@$00@@QEAAXH@Z",
        b"void __cdecl BoolTemplate<1>::Foo<int>(int)",
    );
    test(
        b"??0?$IntTemplate@$0A@@@QAE@XZ",
        b"__thiscall IntTemplate<0>::IntTemplate<0>(void)",
    );
    test(
        b"??0?$IntTemplate@$0A@@@QEAA@XZ",
        b"__cdecl IntTemplate<0>::IntTemplate<0>(void)",
    );
    test(
        b"??0?$IntTemplate@$04@@QAE@XZ",
        b"__thiscall IntTemplate<5>::IntTemplate<5>(void)",
    );
    test(
        b"??0?$IntTemplate@$04@@QEAA@XZ",
        b"__cdecl IntTemplate<5>::IntTemplate<5>(void)",
    );
    test(
        b"??0?$IntTemplate@$0L@@@QAE@XZ",
        b"__thiscall IntTemplate<11>::IntTemplate<11>(void)",
    );
    test(
        b"??0?$IntTemplate@$0L@@@QEAA@XZ",
        b"__cdecl IntTemplate<11>::IntTemplate<11>(void)",
    );
    test(
        b"??0?$IntTemplate@$0BAA@@@QAE@XZ",
        b"__thiscall IntTemplate<256>::IntTemplate<256>(void)",
    );
    test(
        b"??0?$IntTemplate@$0BAA@@@QEAA@XZ",
        b"__cdecl IntTemplate<256>::IntTemplate<256>(void)",
    );
    test(
        b"??0?$IntTemplate@$0CAB@@@QAE@XZ",
        b"__thiscall IntTemplate<513>::IntTemplate<513>(void)",
    );
    test(
        b"??0?$IntTemplate@$0CAB@@@QEAA@XZ",
        b"__cdecl IntTemplate<513>::IntTemplate<513>(void)",
    );
    test(
        b"??0?$IntTemplate@$0EAC@@@QAE@XZ",
        b"__thiscall IntTemplate<1026>::IntTemplate<1026>(void)",
    );
    test(
        b"??0?$IntTemplate@$0EAC@@@QEAA@XZ",
        b"__cdecl IntTemplate<1026>::IntTemplate<1026>(void)",
    );
    test(
        b"??0?$IntTemplate@$0PPPP@@@QAE@XZ",
        b"__thiscall IntTemplate<65535>::IntTemplate<65535>(void)",
    );
    test(
        b"??0?$IntTemplate@$0PPPP@@@QEAA@XZ",
        b"__cdecl IntTemplate<65535>::IntTemplate<65535>(void)",
    );
    test(
        b"??0?$IntTemplate@$0?0@@QAE@XZ",
        b"__thiscall IntTemplate<-1>::IntTemplate<-1>(void)",
    );
    test(
        b"??0?$IntTemplate@$0?0@@QEAA@XZ",
        b"__cdecl IntTemplate<-1>::IntTemplate<-1>(void)",
    );
    test(
        b"??0?$IntTemplate@$0?8@@QAE@XZ",
        b"__thiscall IntTemplate<-9>::IntTemplate<-9>(void)",
    );
    test(
        b"??0?$IntTemplate@$0?8@@QEAA@XZ",
        b"__cdecl IntTemplate<-9>::IntTemplate<-9>(void)",
    );
    test(
        b"??0?$IntTemplate@$0?9@@QAE@XZ",
        b"__thiscall IntTemplate<-10>::IntTemplate<-10>(void)",
    );
    test(
        b"??0?$IntTemplate@$0?9@@QEAA@XZ",
        b"__cdecl IntTemplate<-10>::IntTemplate<-10>(void)",
    );
    test(
        b"??0?$IntTemplate@$0?L@@@QAE@XZ",
        b"__thiscall IntTemplate<-11>::IntTemplate<-11>(void)",
    );
    test(
        b"??0?$IntTemplate@$0?L@@@QEAA@XZ",
        b"__cdecl IntTemplate<-11>::IntTemplate<-11>(void)",
    );
    test(
        b"??0?$UnsignedIntTemplate@$0PPPPPPPP@@@QAE@XZ",
        b"__thiscall UnsignedIntTemplate<4294967295>::UnsignedIntTemplate<4294967295>(void)",
    );
    test(
        b"??0?$UnsignedIntTemplate@$0PPPPPPPP@@@QEAA@XZ",
        b"__cdecl UnsignedIntTemplate<4294967295>::UnsignedIntTemplate<4294967295>(void)",
    );
    test(b"??0?$LongLongTemplate@$0?IAAAAAAAAAAAAAAA@@@QAE@XZ", b"__thiscall LongLongTemplate<-9223372036854775808>::LongLongTemplate<-9223372036854775808>(void)");
    test(b"??0?$LongLongTemplate@$0?IAAAAAAAAAAAAAAA@@@QEAA@XZ", b"__cdecl LongLongTemplate<-9223372036854775808>::LongLongTemplate<-9223372036854775808>(void)");
    test(b"??0?$LongLongTemplate@$0HPPPPPPPPPPPPPPP@@@QAE@XZ", b"__thiscall LongLongTemplate<9223372036854775807>::LongLongTemplate<9223372036854775807>(void)");
    test(b"??0?$LongLongTemplate@$0HPPPPPPPPPPPPPPP@@@QEAA@XZ", b"__cdecl LongLongTemplate<9223372036854775807>::LongLongTemplate<9223372036854775807>(void)");
    test(
        b"??0?$UnsignedLongLongTemplate@$0?0@@QAE@XZ",
        b"__thiscall UnsignedLongLongTemplate<-1>::UnsignedLongLongTemplate<-1>(void)",
    );
    test(
        b"??0?$UnsignedLongLongTemplate@$0?0@@QEAA@XZ",
        b"__cdecl UnsignedLongLongTemplate<-1>::UnsignedLongLongTemplate<-1>(void)",
    );
    test(
        b"??$foo@H@space@@YAABHABH@Z",
        b"int const & __cdecl space::foo<int>(int const &)",
    );
    test(
        b"??$foo@H@space@@YAAEBHAEBH@Z",
        b"int const & __cdecl space::foo<int>(int const &)",
    );
    test(
        b"??$FunctionPointerTemplate@$1?spam@@YAXXZ@@YAXXZ",
        b"void __cdecl FunctionPointerTemplate<&void __cdecl spam(void)>(void)",
    );
    test(b"??$variadic_fn_template@HHHH@@YAXABH000@Z", b"void __cdecl variadic_fn_template<int, int, int, int>(int const &, int const &, int const &, int const &)");
    test(b"??$variadic_fn_template@HHD$$BY01D@@YAXABH0ABDAAY01$$CBD@Z", b"void __cdecl variadic_fn_template<int, int, char, char[2]>(int const &, int const &, char const &, char const (&)[2]");
    test(
        b"??0?$VariadicClass@HD_N@@QAE@XZ",
        b"__thiscall VariadicClass<int, char, bool>::VariadicClass<int, char, bool>(void)",
    );
    test(
        b"??0?$VariadicClass@_NDH@@QAE@XZ",
        b"__thiscall VariadicClass<bool, char, int>::VariadicClass<bool, char, int>(void)",
    );
    test(b"?template_template_fun@@YAXU?$Type@U?$Thing@USecond@@$00@@USecond@@@@@Z", b"void __cdecl template_template_fun(struct Type<struct Thing<struct Second, 1>, struct Second>)");
    test(b"??$template_template_specialization@$$A6AXU?$Type@U?$Thing@USecond@@$00@@USecond@@@@@Z@@YAXXZ", b"void __cdecl template_template_specialization<void __cdecl(struct Type<struct Thing<struct Second, 1>, struct Second>)>(void)");
    test(b"?f@@YAXU?$S1@$0A@@@@Z", b"void __cdecl f(struct S1<0>)");
    test(
        b"?recref@@YAXU?$type1@$E?inst@@3Urecord@@B@@@Z",
        b"void __cdecl recref(struct type1<struct record const inst>)",
    );
    test(b"?fun@@YAXU?$UUIDType1@Uuuid@@$1?_GUID_12345678_1234_1234_1234_1234567890ab@@3U__s_GUID@@B@@@Z", b"void __cdecl fun(struct UUIDType1<struct uuid, &struct __s_GUID const _GUID_12345678_1234_1234_1234_1234567890ab>)");
    test(b"?fun@@YAXU?$UUIDType2@Uuuid@@$E?_GUID_12345678_1234_1234_1234_1234567890ab@@3U__s_GUID@@B@@@Z", b"void __cdecl fun(struct UUIDType2<struct uuid, struct __s_GUID const _GUID_12345678_1234_1234_1234_1234567890ab>)");
    test(
        b"?FunctionDefinedWithInjectedName@@YAXU?$TypeWithFriendDefinition@H@@@Z",
        b"void __cdecl FunctionDefinedWithInjectedName(struct TypeWithFriendDefinition<int>)",
    );
    test(b"?bar@?$UUIDType4@$1?_GUID_12345678_1234_1234_1234_1234567890ab@@3U__s_GUID@@B@@QAEXXZ", b"void __thiscall UUIDType4<&struct __s_GUID const _GUID_12345678_1234_1234_1234_1234567890ab>::bar(void)");
    test(
        b"??$f@US@@$1?g@1@QEAAXXZ@@YAXXZ",
        b"void __cdecl f<struct S, &public: void __cdecl S::g(void)>(void)",
    );
    test(
        b"??$?0N@?$Foo@H@@QEAA@N@Z",
        b"__cdecl Foo<int>::Foo<int><double>(double)",
    );
}

#[test]
fn test_thunks() {
    test(
        b"?f@C@@WBA@EAAHXZ",
        b"[thunk]: public: virtual int __cdecl C::f`adjustor{16}'(void)",
    );
    test(b"??_EDerived@@$4PPPPPPPM@A@EAAPEAXI@Z", b"[thunk]: public: virtual void * __cdecl Derived::`vector deleting dtor'`vtordisp{-4, 0}'(unsigned int)");
    test(
        b"?f@A@simple@@$R477PPPPPPPM@7AEXXZ",
        b"[thunk]: public: virtual void __thiscall simple::A::f`vtordispex{8, 8, -4, 8}'(void)",
    );
    test(
        b"??_9Base@@$B7AA",
        b"[thunk]: __cdecl Base::`vcall'{8, {flat}}",
    );
}

#[test]
fn test_windows() {
    test(b"?bar@Foo@@SGXXZ", b"static void __stdcall Foo::bar(void)");
    test(b"?bar@Foo@@QAGXXZ", b"void __stdcall Foo::bar(void)");
    test(b"?f2@@YIXXZ", b"void __fastcall f2(void)");
    test(b"?f1@@YGXXZ", b"void __stdcall f1(void)");
    test(b"?f5@@YCXXZ", b"void __pascal f5(void)");
}

#[test]
fn test_fuzzed() {
    _ = crate::demangle(
        (&[
            63, 63, 95, 67, 64, 95, 49, 79, 67, 64, 67, 64, 95, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119,
            119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119,
            119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119,
            119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119,
            119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119,
            119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119,
            119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119,
            119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 254,
            255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 1,
            1, 83, 31, 68, 255, 255, 255, 3, 64, 64, 88, 88, 88, 88, 88, 88, 63,
        ])
            .into(),
        Flags::default(),
    );
    _ = crate::demangle(
        (&[
            63, 63, 49, 63, 36, 0, 0, 7, 64, 36, 69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63,
            63, 49, 63, 36, 36, 36, 69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63, 49, 63,
            36, 36, 0, 0, 7, 64, 36, 69, 63, 63, 49, 63, 36, 0, 169, 219, 219, 36, 0, 0, 7, 64, 36,
            69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63, 49, 63, 36, 36, 0, 0, 7, 117,
            117, 36, 36, 117, 117, 117, 66, 117, 117, 117, 117, 117, 117, 119, 117, 117, 117, 117,
            117, 117, 117, 117, 117, 0, 169, 219, 219, 36, 0, 0, 7, 64, 36, 69, 63, 63, 49, 63, 36,
            0, 59, 63, 64, 36, 69, 63, 63, 49, 63, 36, 36, 0, 0, 7, 117, 117, 36, 36, 66, 117, 117,
            117, 117, 117, 117, 117, 117, 117, 119, 117, 117, 117, 117, 117, 117, 117, 117, 117,
            117, 117, 117, 117, 59, 63, 64, 36, 69, 63, 63, 49, 63, 36, 36, 0, 0, 7, 64, 36, 69,
            63, 63, 49, 63, 36, 0, 169, 219, 219, 36, 0, 0, 7, 64, 36, 69, 63, 63, 49, 63, 36, 0,
            59, 63, 64, 36, 69, 63, 63, 49, 63, 36, 36, 0, 0, 7, 117, 117, 36, 36, 117, 117, 117,
            66, 117, 117, 117, 117, 117, 117, 119, 117, 117, 117, 117, 117, 117, 117, 117, 117, 0,
            169, 219, 219, 36, 0, 0, 7, 64, 36, 69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63,
            63, 49, 63, 36, 36, 0, 0, 7, 117, 117, 36, 36, 66, 117, 117, 117, 117, 117, 117, 117,
            117, 117, 119, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117,
            117, 117, 117, 36, 0, 0, 7, 64, 36, 69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63,
            63, 49, 63, 36, 36, 36, 69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63, 49, 63,
            36, 36, 0, 0, 7, 64, 36, 69, 63, 63, 49, 63, 36, 0, 169, 219, 219, 36, 0, 0, 7, 64, 36,
            69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63, 49, 63, 36, 36, 0, 0, 7, 117,
            117, 36, 36, 117, 117, 117, 66, 117, 117, 117, 117, 117, 117, 119, 117, 117, 117, 117,
            117, 117, 117, 117, 117, 0, 169, 219, 219, 36, 0, 0, 7, 64, 36, 69, 63, 63, 49, 63, 36,
            0, 59, 63, 64, 36, 69, 63, 63, 49, 63, 36, 36, 0, 0, 7, 117, 117, 36, 36, 66, 117, 117,
            117, 117, 117, 117, 117, 117, 117, 119, 117, 117, 117, 117, 117, 117, 117, 117, 117,
            117, 117, 117, 117, 59, 63, 64, 36, 69, 63, 63, 49, 63, 36, 36, 0, 0, 7, 64, 36, 69,
            63, 63, 49, 63, 36, 0, 169, 219, 219, 36, 0, 0, 7, 64, 36, 69, 63, 63, 49, 63, 36, 0,
            59, 63, 64, 36, 69, 63, 63, 49, 63, 36, 36, 0, 0, 7, 117, 117, 36, 36, 117, 117, 117,
            66, 117, 117, 117, 117, 117, 117, 119, 117, 117, 117, 117, 117, 117, 117, 117, 117, 0,
            169, 219, 219, 36, 0, 0, 7, 64, 36, 69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63,
            63, 49, 63, 36, 36, 0, 0, 7, 117, 117, 36, 36, 66, 117, 117, 117, 117, 117, 117, 117,
            117, 117, 119, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117,
            117, 117, 117, 117, 117, 126, 117, 117, 117, 117, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 117, 117, 117, 117, 117, 117, 117, 117, 1, 117, 117, 117, 117,
            117, 117, 117, 117, 64, 36, 69, 63, 63, 48, 63, 36, 0, 169, 219, 219, 255, 69, 63, 63,
            49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63, 95, 49, 65, 63, 1, 64, 63, 255, 255, 255,
            255, 64, 64, 57, 64, 63, 169, 95, 65, 63, 63, 49, 79, 64, 63, 36, 63, 88, 70, 64, 63,
            36, 63, 88, 74, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 69,
            63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63, 95, 95, 65, 63, 1, 64, 63, 255, 255,
            255, 255, 64, 64, 57, 64, 63, 169, 95, 65, 63, 63, 255, 255, 255, 255, 64, 64, 57, 64,
            63, 169, 95, 65, 63, 63, 49, 79, 64, 64, 57, 64, 127, 169, 36, 69, 95, 64, 64, 57, 64,
            95, 63, 63, 49, 79, 64, 64, 57, 64, 63, 169, 36, 69, 95, 64, 64, 57, 64, 95, 63, 169,
            36, 69, 95, 64, 64, 57, 64, 42, 155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 149, 149, 149, 149, 149, 149, 149, 149, 149, 117, 117,
            117, 117, 117, 117, 117, 155, 0, 0, 0, 0, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63,
            49, 63, 36, 36, 0, 0, 7, 117, 117, 117, 117, 117, 117, 117, 117, 126, 117, 117, 117,
            117, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 117, 117, 117, 117,
            117, 117, 117, 117, 1, 117, 117, 117, 117, 117, 117, 117, 117, 64, 36, 69, 63, 63, 48,
            63, 36, 0, 169, 219, 219, 255, 69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63,
            95, 49, 65, 63, 1, 64, 63, 255, 255, 255, 255, 64, 64, 57, 64, 63, 169, 95, 65, 63, 63,
            49, 79, 64, 63, 36, 63, 88, 70, 64, 63, 36, 63, 88, 74, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63,
            63, 95, 95, 65, 63, 1, 64, 63, 255, 255, 255, 255, 64, 64, 57, 64, 63, 169, 95, 65, 63,
            63, 255, 255, 255, 255, 64, 64, 57, 64, 63, 169, 95, 65, 63, 63, 49, 79, 64, 64, 57,
            64, 127, 169, 36, 117, 117, 126, 117, 117, 117, 117, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 117, 117, 117, 117, 117, 117, 117, 117, 1, 117, 117, 117, 117,
            117, 117, 117, 117, 64, 36, 69, 63, 63, 48, 63, 36, 0, 169, 219, 219, 255, 69, 63, 63,
            49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63, 95, 49, 65, 63, 1, 64, 63, 255, 255, 255,
            255, 64, 64, 57, 64, 63, 169, 95, 65, 63, 63, 49, 79, 64, 63, 36, 63, 88, 70, 64, 63,
            36, 63, 88, 74, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 69,
            63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63, 95, 95, 65, 63, 1, 64, 63, 255, 255,
            255, 255, 64, 64, 57, 64, 63, 169, 95, 65, 63, 63, 255, 255, 255, 255, 64, 64, 57, 64,
            63, 169, 95, 65, 63, 63, 49, 79, 64, 64, 57, 64, 127, 169, 36, 69, 95, 64, 64, 57, 64,
            95, 63, 63, 49, 79, 64, 64, 57, 64, 63, 169, 36, 69, 95, 64, 64, 57, 64, 95, 63, 169,
            36, 69, 95, 64, 64, 57, 64, 42, 155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 149, 149, 149, 149, 149, 149, 149, 149, 149, 117, 117,
            117, 117, 117, 117, 117, 155, 0, 0, 0, 0, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63,
            49, 63, 36, 36, 0, 0, 7, 117, 117, 117, 117, 117, 117, 117, 117, 126, 117, 117, 117,
            117, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 117, 117, 117, 117,
            117, 117, 117, 117, 1, 117, 117, 117, 117, 117, 117, 117, 117, 64, 36, 69, 63, 63, 48,
            63, 36, 0, 169, 219, 219, 255, 69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63, 63,
            95, 49, 65, 63, 1, 64, 63, 255, 255, 255, 255, 64, 64, 57, 64, 63, 169, 95, 65, 63, 63,
            49, 79, 64, 63, 36, 63, 88, 70, 64, 63, 36, 63, 88, 74, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 69, 63, 63, 49, 63, 36, 0, 59, 63, 64, 36, 69, 63,
            63, 95, 95, 65, 63, 1, 64, 63, 255, 255, 255, 255, 64, 64, 57, 64, 63, 169, 95, 65, 63,
            63, 255, 255, 255, 255, 64, 64, 57, 64, 63, 169, 95, 65, 63, 63, 49, 79, 64, 64, 57,
            64, 127, 169, 36, 69, 95, 64, 64, 57, 64, 95, 63, 63, 49, 79, 64, 64, 57, 64, 63, 169,
            36, 69, 95, 64, 64, 57, 64, 95, 63, 169, 36, 69, 95, 64, 64, 57, 64, 42, 155, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149,
            149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149,
            149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149,
            149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149,
            149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149, 149,
            149, 149, 65, 65, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63,
        ])
            .into(),
        Flags::default(),
    );
}

#[test]
fn test_no_this_type() {
    let test_option = |mangled_name: &[u8], demangled_name: &[u8]| {
        do_test(mangled_name, demangled_name, false, Flags::NO_THISTYPE);
    };

    test_option(
        b"?world@hello@@QEBAXXZ",
        b"public: void __cdecl hello::world(void)",
    );
    test_option(
        b"?world@hello@@QECAXXZ",
        b"public: void __cdecl hello::world(void)",
    );
    test_option(
        b"?world@hello@@QEIAAXXZ",
        b"public: void __cdecl hello::world(void)",
    );
    test_option(
        b"?world@hello@@QEFAAXXZ",
        b"public: void __cdecl hello::world(void)",
    );
    test_option(
        b"?a@FTypeWithQuals@@3U?$S@$$A8@@BAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::a",
    );
    test_option(
        b"?b@FTypeWithQuals@@3U?$S@$$A8@@CAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::b",
    );
    test_option(
        b"?c@FTypeWithQuals@@3U?$S@$$A8@@IAAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::c",
    );
    test_option(
        b"?d@FTypeWithQuals@@3U?$S@$$A8@@GBAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::d",
    );
    test_option(
        b"?e@FTypeWithQuals@@3U?$S@$$A8@@GCAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::e",
    );
    test_option(
        b"?f@FTypeWithQuals@@3U?$S@$$A8@@IGAAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::f",
    );
    test_option(
        b"?g@FTypeWithQuals@@3U?$S@$$A8@@HBAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::g",
    );
    test_option(
        b"?h@FTypeWithQuals@@3U?$S@$$A8@@HCAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::h",
    );
    test_option(
        b"?i@FTypeWithQuals@@3U?$S@$$A8@@IHAAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::i",
    );
    test_option(
        b"?j@FTypeWithQuals@@3U?$S@$$A6AHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::j",
    );
    test_option(
        b"?k@FTypeWithQuals@@3U?$S@$$A8@@GAAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::k",
    );
    test_option(
        b"?l@FTypeWithQuals@@3U?$S@$$A8@@HAAHXZ@1@A",
        b"struct FTypeWithQuals::S<int __cdecl(void)> FTypeWithQuals::l",
    );
}

#[test]
fn test_no_leading_underscores() {
    let test_option = |mangled_name: &[u8], demangled_name: &[u8]| {
        do_test(
            mangled_name,
            demangled_name,
            false,
            Flags::NO_LEADING_UNDERSCORES,
        );
    };

    test_option(
        b"?unaligned_foo5@@YAXPIFAH@Z",
        b"void cdecl unaligned_foo5(int unaligned *restrict)",
    );
    test_option(
        b"?beta@@YI_N_J_W@Z",
        b"bool fastcall beta(__int64, wchar_t)",
    );
    test_option(b"?f5@@YCXXZ", b"void pascal f5(void)");
    test_option(
        b"?j@@3P6GHCE@ZA",
        b"int (stdcall *j)(signed char, unsigned char)",
    );
    test_option(
        b"?mbb@S@@QAEX_N0@Z",
        b"public: void thiscall S::mbb(bool, bool)",
    );
    test_option(b"?vector_func@@YQXXZ", b"void vectorcall vector_func(void)");
}
