use rand::{thread_rng};
use rand::seq::SliceRandom;

#[derive(Serialize,Deserialize, Debug)]
pub struct Cell {
    pub id  : usize,
    pub open: bool,
    pub mine: bool,
    pub flag: bool,
    pub neighborhood: Vec<usize>,
    pub mines_in_neighborhood: usize,
}

pub struct GameField {
    pub cells: Vec<Cell>,
    pub game_state: u8,
}

impl GameField {
    pub fn create(px:u8,py:u8,pm:u8) -> GameField {
	let mut cells : Vec<Cell> = vec![];
	let y = px as usize;
	let x = py as usize;
	let m = pm as usize;
	for i in 0..(x*y) as usize {
	    let neighborhood : Vec<usize>;
	    if i == 0 			   {neighborhood = vec![1,x,x+1];}
	    else if i == x-1 		   {neighborhood = vec![x-2,2*x-1,2*x-2];}
	    else if i == (y-1)*x 	   {neighborhood = vec![(y-2)*x,(y-2)*x+1,(y-1)*x+1];}
	    else if i == (y*x)-1 	   {neighborhood = vec![(y*x)-2,((y-1)*x)-1,(y-1)*x-2];}
	    else if i>0 && i<x-1 	   {neighborhood = vec![i-1,(i+x)-1,i+x,i+x+1,i+1];}
	    else if i % x == 0   	   {neighborhood = vec![i+x,i+x+1,i+1,(i-x)+1,i-x];}
	    else if (i % x) == (x-1) 	   {neighborhood = vec![i-x,(i-x)-1,i-1,(i+x)-1,i+x];}
	    else if i>(y-1)*x && i<(y*x)-1 {neighborhood = vec![i-1,(i-x)-1,i-x,(i-x)+1,i+1];}
	    else 			   {neighborhood = vec![i-x,(i-x)-1,i-1,(i+x)-1,i+x,i+x+1,i+1,(i-x)+1];}
	    cells.push(Cell{id:i,open:false,mine:false,flag:false,neighborhood,mines_in_neighborhood:0});
	}

	let mut meta : Vec<usize> = (0..(x*y) as usize).collect::<Vec<usize>>();
	meta.shuffle(&mut thread_rng());
	for i in meta.into_iter().take(m){cells[i].mine = true;}

	(0..(x*y)).into_iter().for_each(|i| cells[i].mines_in_neighborhood = cells[i].neighborhood.iter().filter(|id| cells[**id].mine).count());

	GameField {cells,game_state:0}
    }

    pub fn open(&mut self,id:usize){
	if self.cells[id].open {return;}
	self.cells[id].open = true;
	if self.cells[id].mine {self.game_state = 1; return;}
	if self.cells[id].mines_in_neighborhood > 0 {return;}
	self.cells[id].neighborhood.clone().iter().for_each(|cell| self.open(*cell));
    }

    pub fn flag(&mut self,id:usize){
	self.cells[id].flag = !self.cells[id].flag; 
    }

    pub fn get_cells(&mut self) -> &Vec<Cell>{
	&self.cells
    }

    pub fn is_running(&mut self) -> bool {
    	self.game_state == 0 
    }

    pub fn is_lost(&mut self) -> bool {
    	self.game_state == 1
    }

    pub fn is_won(&mut self) -> bool {
    	self.game_state == 2
    }

    pub fn check_game_won(&mut self) {
	for cell in &self.cells {
	    if !cell.open && !cell.mine {return;}
	}
	self.game_state = 2;
    }
}
