

pub(crate) fn run(diskMap: &Vec<char>){

    let mut result:u64 = 0;

    //println!("Disk Map: {:?}", diskMap);

    let mut uncompressed: Vec<String> = Vec::new();

   for (index, c) in  diskMap.iter().enumerate() {

       let mut cNum = c.to_digit(10).unwrap();

       if index % 2 == 0 {
           let fileId = index as u32 / 2;

           //file
           while cNum > 0 {
               let fileIdString =   fileId.to_string();
               uncompressed.push( fileIdString);
               cNum -= 1;
           }
       }
       else {
           //free space
           while cNum > 0 {
               uncompressed.push(".".to_string());
               cNum -= 1;
           }
       }

    }

    //println!("Uncompressed: {:?}", uncompressed);

    let mut defragmented: Vec<String> = uncompressed;
    let mut i = 0;
    let mut j = defragmented.len()-1;

    while i < j {
        if defragmented[i] != "." { i +=1;}
        else if defragmented[j] == "." { j-=1;}
        else {
            defragmented.swap(i, j);
            i+=1;
            j-=1;
        }
    }

    //println!("Uncompressed: {:?}", defragmented);

    for (i,c) in defragmented.iter().enumerate() {
        if *c == "." {break;}
        result += i as u64 * c.parse::<u64>().unwrap();
    }


    println!("Result: {}", result);

}