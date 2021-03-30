<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <?php
    function lessThanOrEqualToZero($num){
        return $num <=0;
    };
    
    function testZeroOrNegative() {
    echo (lessThanOrEqualToZero(5) == false); echo ( "5 is not more than zero");
    echo (lessThanOrEqualToZero(0) == true); echo ( "0 is equal zero");
    echo (lessThanOrEqualToZero(-5) == true); echo ( "-5 is less than zero");
    echo (lessThanOrEqualToZero(1) == false); echo ( "1 is not  more than zero");
    echo (lessThanOrEqualToZero(2) == false); echo ( "2 is not  more than zero");
    echo (lessThanOrEqualToZero(10000) == false); echo ( "10000 is not  more than zero");
    echo (lessThanOrEqualToZero(0.5) == false); echo ("0.5 is not more than zero");       }
      testZeroOrNegative()
    ?>
</body>
</html>