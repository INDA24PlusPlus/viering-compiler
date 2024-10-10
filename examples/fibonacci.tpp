var a = 0!
var b = 1!
var n = 0!

loop {
    print b!

    if(n == 10){
        break!
    }

    var c = a + b!
    a = b!
    b = c!
    n = n + 1!
}
