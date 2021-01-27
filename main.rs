fn main() {
    let mut a = vec![1,22,13,134];
    let mut b = vec![3,5,5,7];
    normalize(&mut a);
    print!("a:");
    print(&a);
    print!("b:");
    print(&b);
    let c = multiply(&a,&b);
    print!("c:");
    print(&c);
    
}

//벡터 형식의 숫자 자리 맞추기
fn normalize(a: &mut Vec<i32>){
    let mut num=0;
    loop{
        if a[num]>10{
            if num==a.len()-1 {a.push(0);}
            a[num+1]+=a[num]/10;
            a[num]%=10;
        }
        num += 1;
        if num==a.len() {break;}
    }
    for i in (0..a.len()).rev(){
        if a[i]==0 {a.pop();}
        else {break;}
    }
}

//벡터 형식의 숫자 출력
fn print(a: &Vec<i32>){
    for num in a.iter().rev(){
        print!("{}",num);
    }
    println!();
}

//벡터 형식의 두 수 곱해서 새 벡터 만든 후 반환
fn multiply(a: &Vec<i32>,b: &Vec<i32>) -> Vec<i32>{
    let mut ret = Vec::new();
    for i in (0..a.len()+b.len()+1){
        ret.push(0);
    }
    for i in (0..a.len()){
        for j in (0..b.len()){
            ret[i+j] += a[i]*b[j];
        }
    }
    normalize(&mut ret);
    ret
}

