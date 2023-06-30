const readline = require('readline');

function isPrime(n){
  var divisor = 2;

  while (n > divisor){
    if(n % divisor == 0){
     return false; 
    }
    else
      divisor++;
  }
  return true;
}

function main() {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });

  rl.question("Enter a number: ", function(input) {
    // Convert the input to an integer
    input = parseInt(input);

    // Call the isPrime function to check if the number is prime
    if (isPrime(input)) {
      console.log(input + " is a prime number.");
    } else {
      console.log(input + " is not a prime number.");
    }

    rl.close();
  });
}

// Call the main function
main();
