<?php
function println(...$args) {
    echo implode('', $args)."\n";
}
function print_(...$args) {
    echo implode('', $args);
}
function input($q='') {
    echo $q;
    $ret = '';
    $buf = '';
    while (strpos(" \r\n\t", $buf=fread(STDIN, 1)) === false) {
        $ret .= $buf;
    }
    return $ret;
}
function input_line($q='') {
    echo $q;
    $ret = trim(fgets(STDIN));
    return $ret;
}
$_sin_cache = [];
$_sinl_cache = [];
function static_input($etag, $q='') {
    return $_sin_cache[$etag] ?: $_sin_cache[$etag]=input($q);
}
function static_input_line($etag, $q='') {
    return $_sinl_cache[$etag] ?: $_sinl_cache[$etag]=input_line($q);
}
function tup(...$args) {
    return $args;
}
function vec(...$args) {
    return $args;
}
function until($begin, $end) {
    return range($begin, $end);
}
function sum($c) {
    return array_sum($c);
}
function map($c, $fn) {
    return array_map($c, $fn);
}
function cat($c1, $c2) {
    return array_merge($c1, $c2);
}
function each_($c, $fn) {
    foreach ($c as $v) {
        $fn($v);
    }
}
function fold($c, $fn) {
    return array_reduce($c, $fn);
}
function bfold($i, $c, $fn) {
    return array_reduce($c, $fn, $i);
}
function filter($c, $fn) {
    return array_filter($c, $fn);
}
function integrate($c, $fn) {
    $ret = [$c[0]];
    for ($i=1; $i < count($c); $i++) { 
        array_push($ret, $fn(end($ret), $c[$i]));
    }
    return $ret;
}
function wait($t) {
    sleep($t/1000);
}
function get_time($fn, ...$args) {
    $st = strtotime(date("Y-m-d H:i:s.u"));
    $fn(...$args);
    $ed = strtotime(date("Y-m-d H:i:s.u"));
    echo ($ed - $st) . ".000000 second(s) spent.\n";
}
function make_string($s) {
    return strval($s);
}
function stoi($s) {
    return intval($s);
}
?>