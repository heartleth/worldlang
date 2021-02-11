def println(*a)
    puts(a.join(''))
end

(println.class.name=='Proc'?println.("Hello, world!"):println("Hello, world!"));

# Shit, fuck up!