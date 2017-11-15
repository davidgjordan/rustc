
fn factorial(n:i32)->i32{

 if n == 1{
 	return 1;
 }else{
 	return n*factorial(n-1);
 } 

}

fn esPar(n:i32){
	if(n%2==0){
	println!("Es par");
		 
	}else{
	println!("Es impar");

	}
}

	
	fn isValid(d:i32)->bool{
	    let mut res = d;
		let anio = res/10000;
		res = res%10000;
		let mes = res/100;
		res = res % 100; 
		let dia = res;

		println!("Anio:{}",anio );
		println!("mes:{}",mes );
		println!("dia:{}",dia );




		return true;

	}
	fn isAnioValid(n:i32)->bool{
		//si es multiplo de 4 el anio entonces el mes de febrero tien un
		//dia mas 
		if(n>1900 && n < 2018){
			true
		}else {
			false
		}

	}
	fn isMesValid(m:i32, a:i32)->bool{
		if(m>0 && m < 12 {
			true
		}else {
			false
		}

	}

	fn isDiaValid(n:i32)->bool{
		if(n>0 && n < 31){
			true
		}else {
			false
		}

	}
	fn isAnioBiciesto(n:i32){
		/*si es multiplo de cuatro es anio biciesto*/
	}

fn main() {
	//variables let como auto en c++->  let obliga  a inicializar mis variables
	let r = factorial(8);
	let p = esPar(6);


	//println!("FActorial:{:?}",r );
	//println!("Hello:{}",r );
	

	let d = 20171115;

	if(isValid(d)){
		println!("Imprimo en formato normal");
	}else{
		println!("Invalid date");
	}





}












