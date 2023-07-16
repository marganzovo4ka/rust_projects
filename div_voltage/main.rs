use std::io;

fn input () -> f32 {

    loop{
    let mut x = String::new();

    io::stdin().read_line(  &mut x).expect("failed to read line");

    let a: f32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_)=> continue,
    };
    return a
    }   
}

fn main() {
   
    println!("enter the input voltage");

    let u_in  = input();

    println!("enter the output voltage");

    let u_out  = input();

    if u_in / 2.0 == u_out{
        println!("any resistors with the same resistance values");
        return;
    }
    
    if u_in/u_out - 1.0 > 10.0 || u_out/u_in - 1.0 > 0.1{
        println!("the resistance ratio should be < 10 or > 0.1, but with these values ​​this condition is violated");
        return;
    }
    if u_in / 2.0 < u_out{
        return;
    }
    else{

    let mut e96:Vec<f32> = vec![100.00, 102.00, 105.00, 107.00, 110.00, 113.00,
    115.00, 118.00, 121.00, 124.00, 127.00, 130.00,
    133.00, 137.00, 140.00, 143.00, 147.00, 150.00,
    154.00, 158.00, 162.00, 165.00, 169.00, 174.00,
    178.00, 182.00, 187.00, 191.00, 196.00, 200.00,
    205.00, 210.00, 215.00, 221.00, 226.00, 232.00,
    237.00, 243.00, 249.00, 255.00, 261.00, 267.00,
    274.00, 280.00, 287.00, 294.00, 301.00, 309.00,
    316.00, 324.00, 332.00, 340.00, 348.00, 357.00,
    365.00,	374.00,	383.00,	392.00,	402.00,	412.00,
    422.00, 432.00, 442.00, 453.00, 464.00, 475.00,
    487.00, 499.00, 511.00, 523.00, 536.00, 549.00,
    562.00, 576.00, 590.00, 604.00, 619.00, 634.00,
    649.00, 665.00, 681.00, 698.00, 715.00, 732.00,
    750.00, 768.00, 787.00, 806.00, 825.00, 845.00,
    866.00, 887.00, 909.00, 931.00, 953.00, 976.00];
    
    let mut values:Vec<(f32,f32)> = Vec::new();
    let mut flag: bool = false;

    for i in 0..e96.len(){
        e96.push(e96[i]*10.0);
    }

    for i in 0..e96.len()/2{
        let r2 = e96[i];
        let r1 = (u_in/u_out - 1.0)*r2;

        if find(&e96, r1){
            flag = true;
            let value = (r1,r2);
            values.push(value);
        }
        else {
            continue;
        } 
    }

    if flag {
        println!("Exact match was found!");
        for i in 0..values.len(){
            println!("R1 : {} \t R2 : {}", values[i].0, values[i].1);
        }
    }
    else {
        let mut value_less:(f32, f32) = (0.0,0.0);
        let mut value_biger:(f32, f32) = (0.0,0.0);
        let mut less_ratio:f32 = 100000.0;
        let mut biger_ratio:f32 = -100000.0;
        let mut u_approx_less:f32 = u_out;
        let mut u_approx_biger:f32 = u_out;
        println!("No exact matches were found");
        for i in 0..e96.len()/2{
            let r2 = e96[i];
            let mut r1 = (u_in/u_out - 1.0)*r2;
            let mut less_index:usize = 1000;
            let mut biger_index:usize = 1000;
            if find_approx(&e96, r1, &mut less_index, &mut biger_index){
                if less_index < 200{
                    r1 = e96[less_index];
                    u_approx_less = u_in *r2/(r1+r2);
                    if u_out - u_approx_less < less_ratio{
                        value_less = (r1,r2);
                        less_ratio = u_out - u_approx_less;
                    }
                }
                if biger_index < 200{
                    r1 = e96[biger_index];
                    u_approx_biger = u_in *r2/(r1+r2);
                    if u_out - u_approx_biger > biger_ratio{
                        value_biger = (r1,r2);
                        biger_ratio = u_out - u_approx_biger;
                    }
                }
            }
        }
if value_less.0 != 0.0 && value_biger.0 != 0.0{
    println!("desired output voltage: {u_out} V\n
Bigger ratio \n
devider information:
Input voltage: {u_in} V 
R1: {} Ohms 
R2: {} Ohms 
===================================
Output Voltage: {} 
Avsolute mistake: {} 
Relative mistake: {} % \n", value_less.0, value_less.1, u_approx_less, u_approx_less - u_out, (u_approx_less - u_out)/u_out * 100.0);

println!("\n Smaller ratio \n
devider information:
Input voltage: {u_in} V 
R1: {} Ohms 
R2: {} Ohms 
===================================
Output Voltage: {} 
Avsolute mistake: {} 
Relative mistake: {} %", value_biger.0, value_biger.1, u_approx_biger, u_approx_biger - u_out, (u_approx_biger - u_out)/u_out * 100.0)   
        }    
    }
}

}


fn find(arr: &Vec<f32>, find:f32) -> bool{
    let mut l:usize = 0;
    let mut r  = arr.len()-1;
    let mut mid = r/2;
    while l+1 != r{
        if arr[mid] < find{
            l = mid;
            mid = (l+r)/2;
        }
        else if arr[mid] > find{
            r = mid;
            mid = (l+r)/2;
        }
        else if arr[mid] == find{
            return true;
        }
    }
    if l+1 == r {
        if arr[l] == find{
            return true
        }
        else if arr[r] == find{
            return true
        }
        else {
            return false
        }
    }
    else{
        return false;
    }
   
}

fn find_approx(arr: &Vec<f32>, find:f32, less_index: &mut usize, biger_index: &mut usize) -> bool{
    let mut l:usize = 0;
    let mut r  = arr.len()-1;
    let mut mid = r/2;
    while l+1 != r{
        if arr[mid] < find{
            l = mid;
            mid = (l+r)/2;
        }
        else if arr[mid] > find{
            r = mid;
            mid = (l+r)/2;
        }
    }
    
        if (arr[l] - find).abs() < 1.0{
            if arr[l] - find > 0.0{
            *less_index = l;
            return true
            }
            else{
                *biger_index = l;
                return true;
            }
        }
        else if (arr[r] - find).abs() < 1.0{
            if arr[r] - find > 0.0{
                    *less_index = r;
                return true
                }
                else{
                    *biger_index = r;
                    return true
                }
        }
        else {
            return false
        }  
}


