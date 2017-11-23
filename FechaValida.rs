
fn getAnio(date:i32)->i32{

	return date/10000;
}
fn getMonth(date:i32)->i32{
	return (date%10000)/100;
}
fn getDay(date:i32)->i32{
	return (date%10000)%100;
}
	
	fn isValidDate(date:i32)->bool{
		let anio = getAnio(date);
		let mes = getMonth(date);
		let dia = getDay(date);


		//si tenemos un ano correcto y un mes correcto segun a sus dias
		if(isValidYear(anio) && isValidMonth(dia,mes, anio)){
			return true;
		}else{
			return false;
		}
	}


	fn isValidYear(a:i32)->bool{
		//si es multiplo de 4 el anio entonces el mes de febrero tien un
		//dia mas 
		if(a>1900 && a < 2018){
			true
		}else {
			false
		}

	}

	fn isValidMonth(d:i32,m:i32, a:i32)->bool{
		//si es un mes valido
		if(m > 0 && m < 12){
			//si es un mes de 31 dias
			if(m == 1 || m == 3 || m == 5 ||m == 7 ||m == 8 ||m == 10 ||m == 12 ){
					return isValidDay(d,31);
				
				//si son los restantes meses exepto mes 2 solo 30 dias
			}else if(m != 2){
					return isValidDay(d,30);
				//si es el mes dos comprobamos si no es anio biciesto
			}else{
				//si es anio biciesto mes dos entre 0 a 29
				if(isYearBiciesto(a)){

					return isValidDay(d,29);
					
				//si no es biciesto mes dos entre 0 y 28 dias
				}else{
					
					return isValidDay(d,28);
				}
			}

			//si no es un mes valido
		}else{
			return false;
		}
	}

	fn isValidDay(d:i32, maxDias:i32)->bool{
		/*Dia valido si esta etre 0 y 31 exepto si es mes dos el anio no tiene q ser biciesto
		si el anio es biciesto el mes dos tiene q tener 29 dias*/
		if(d > 0 && d <= maxDias ){
				return true;
		}else{
				return false;
		}

	}

	fn isYearBiciesto(a:i32)->bool{
		/*si es multiplo de cuatro es anio biciesto*/
		/*cada 4 anios es biciesto execto si ese anio es multiplo de 100*/
		//primero si es un anio valido
		if(isValidYear(a)){
			//si es biciesto o no
			if(a % 4 == 0 && a % 100 != 0 ){
				return true;
			}else{
				return false;
			}
		}else{
			return false;
		}
	}

fn main() {
	
	let date = 20171115;

	if(isValidDate(date)){
		
		println!("Fecha correcta: ");
		println!("Fecha anio: {}",getAnio(date));
		println!("Fecha mes: {}",getMonth(date));
		println!("Fecha dia: {}",getDay(date));

	}else{
		println!("Invalid date");
	}


}












