
#ifndef _WLIBSTD_
#define _WLIBSTD_
#include <type_traits>
#include <algorithm>
#include <iostream>
#include <fstream>
#include <numeric>
#include <vector>
#include <string>
#include <thread>
#include <future>
#include <chrono>
#include <regex>
#include <tuple>
#include <map>
#ifdef __cpp_lib_ranges
#include <ranges>
#endif
#define dthread(t) std::thread((t)).detach()
struct __instead_of_void { friend std::ostream& operator<<(std::ostream& os, __instead_of_void _) { return os << "void"; }};
template<class T> struct __gt { using t = T; };
template<> struct __gt<void> { using t = __instead_of_void; };
template<class T> using __gt_t=typename __gt<T>::t;
#define __into_f(e) ([&](){return e;})
#define safe(e) __vr1(__into_f(e))
#define print_safe(e) __pr1(__into_f(e))
template<class T>void _print(T e) {std::cout << print_safe(e);}
template<class F>inline constexpr auto __vr1(const F& e) { if constexpr (std::is_same<decltype(e()), void>::value){e(); return __instead_of_void();} else return e(); }
template <class T>struct __lv {template <class U>constexpr static auto less_than_test(const U u) -> decltype(std::cout << u, char(0)){return 0;}constexpr static int less_than_test(...) {return 0;}
constexpr static const bool value = (sizeof(less_than_test(std::declval<T>())) == 1);};
template <class T>std::string __pa(T& e) { std::ostringstream oss; oss << &e; return oss.str(); }
template <class T>std::string __pa(T&& e) { return "rvalue"; }
template<class F>inline constexpr auto __pr1(const F& e) {if
constexpr (std::is_same<decltype(e()), void>::value) { e(); return __instead_of_void(); }else if constexpr (!__lv<decltype(e())>::value) {auto&&v=e();std::ostringstream oss;oss<<"("<<
typeid(e()).name() << ":" << sizeof(decltype(e())) <<")";return oss.str();}else return e();}
struct __constructor{__constructor(){std::cout.tie(0);std::ios_base::sync_with_stdio(0);std::cout<<std::boolalpha;}}__Construct;
typedef char i1; typedef short i2; typedef long i4; typedef long long i8;
typedef unsigned char u1; typedef unsigned short u2; typedef unsigned long u4; typedef unsigned long long u8;
typedef float f4; typedef double f8; typedef long double ld;
typedef const char ci1; typedef const unsigned char cu1; typedef const short ci2; typedef const long ci4; typedef const long long ci8;
typedef const unsigned short cu2; typedef const unsigned long cu4; typedef const unsigned long long cu8;
typedef const float cf4; typedef const double cf8; typedef const long double cld;
using std::vector; using std::string; using std::stoi; using namespace std::string_literals; using namespace std::chrono_literals;using std::async;using std::move;
template<class F, class...T>void get_time(F f, T...a) {
auto st = std::chrono::system_clock::now(); f(a...); std::chrono::duration<double>t = std::chrono::system_clock::now() - st; std::cout << t.count() << " second(s) spent." << std::endl;}
std::string input_line(std::string a = "") { std::string b; std::cout << a; getline(std::cin, b); return b; }
std::string input(std::string a = "") { std::string b; std::cout << a; std::cin >> b; return b; }
std::string static_input(int etag, std::string a = "") { static std::map<int, std::string>memoi; if (memoi.count(etag)) { return memoi[etag]; } std::string b; std::cout << a; std::cin >> b; memoi.insert(std::make_pair(etag, b)); return b; }
std::string static_input_line(int etag, std::string a = "") { static std::map<int, std::string>memoi; if (memoi.count(etag)) { return memoi[etag]; } std::string b; std::cout << a; getline(std::cin, b); memoi.insert(std::make_pair(etag, b)); return b; }
template<class...T>auto tup(T...arg)->std::tuple<T...> { return std::tuple<T...>(arg...); }
template<class T>class __folder {
public:T c; template<class E>__folder& operator<< (E a) { c.push_back(a); return*this; }};
template<class...T>void print(const T&...arg) { (([&](){_print(arg);})(), ...); }
template<class...T>void println(const T&...arg) { (([&](){_print(arg);})(), ...); std::cout << std::endl; }
template<class T, class...R>class __gft { public:typedef T CORE; };
template<class...T>std::vector<typename __gft<T...>::CORE> vec(T...arg) { __folder<std::vector<typename __gft<T...>::CORE>> r; return (r << ... << arg).c; }
template<class T>std::string make_string(T a) { std::stringstream k; k << a; return k.str(); }
template<class T, class F>auto map(T c, F f)->std::vector<decltype(f(*c.begin()))> { std::vector<decltype(f(*c.begin()))>g; for (const auto& i : c) { g.push_back(f(i)); }return g; }
template<class T, class F>void each(T c, F f) { std::for_each(c.begin(), c.end(), f); }
template<class T, class F>auto filter(T c, F f)->std::vector<typename T::value_type> { std::vector<typename T::value_type>a; for (const auto& i : c)if (f(i))a.push_back(i); return a; }
template<typename T, typename F>auto integrate(T c, F f)->std::vector<decltype(f(*c.begin(), *c.begin()))> { typedef typename T::value_type vt; auto iter = c.begin(); vt rdc = *iter; std::vector<vt>ret;ret.push_back(rdc); iter++;
for (; iter != c.end(); iter++) { rdc = f(rdc, *iter); ret.push_back(rdc); }return ret; }
template<typename T, typename F, typename vt = typename T::value_type>auto fold(const T & c, const F & f)->decltype(f(*c.begin(), *c.begin())) { auto iter = c.begin(); vt rdc = *iter; iter++; std::vector<vt>ret; for (; iter != c.end(); iter++) { rdc = f(rdc, *iter); }return rdc; }
template<typename T, typename F, typename vt>auto bfold(const vt & d, const T & c, const F & f)->decltype(f(vt(), *c.begin())) { auto iter = c.begin(); vt rdc = d; std::vector<vt>ret; for (; iter != c.end(); iter++) { rdc = f(rdc, *iter); }return rdc; }
template<class T1, class T2>std::vector<typename T1::value_type> cat(T1 a, T2 b) { std::vector<typename T1::value_type> ret(a.begin(), a.end()); ret.insert(ret.end(), b.begin(), b.end()); return ret; }
template<class T, class F = std::less<typename T::value_type>>typename T::iterator max(T&& e, F f = F()) {auto first = e.begin();auto last = e.end();if (first == last) return last;typename T::iterator largest = first;++first;
for (; first != last; ++first) {if (f(*largest, *first)) {largest = first;}}return largest;}
template<class T>typename T::value_type sum(T&& c) {return std::accumulate(c.begin(), c.end(), typename T::value_type());}
template<class T>T wait(T s) { std::this_thread::sleep_for(std::chrono::duration<int, std::milli>(s)); return s; }
class range {private:int start; int End; int diff; public:typedef int value_type;typedef const int& const_reference;typedef int&reference;
range(int _end) { start = 0; End = _end; diff = 1; }
range(int _start, int _end, int _diff = 1) { start = _start; End = _end; diff = _diff; }
class iterator {
private:int _diff; public:int _val; iterator(int v, int d) :_val(v), _diff(d) {}
auto operator++()->iterator& { _val += _diff; return *this; }
inline int operator*() { return _val; }
int operator==(iterator i) { return (i._val == _val); }
int operator!=(iterator i) { return (i._val >= _val + _diff); }};
inline auto begin()->iterator { return iterator(start, diff); }
inline auto end()->iterator { return iterator(End + diff, diff); }};
std::pair<std::string, std::string> pr(const std::string& a, const std::string& b) {return std::pair<std::string, std::string>(a, b);}
inline range until(i4 a, i4 b) { return range(a, b); }
#ifdef __cpp_lib_ranges
namespace srv = std::ranges::views;
namespace sr = std::ranges;
#endif
#endif
