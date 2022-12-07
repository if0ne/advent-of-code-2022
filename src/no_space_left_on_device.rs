use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::no_space_left_on_device::tree::{Arena, Cursor};
use crate::problem::Problem;

pub struct NoSpaceLeftOnDevice;

impl Problem for NoSpaceLeftOnDevice {
    type InputData = Cursor<'static, u64>;
    type OutputData = u64;

    fn read_file(filename: impl AsRef<Path>) -> Self::InputData {
        let file = std::fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        //Skip first line "$ cd /"
        let lines = reader
            .lines()
            .map(Result::unwrap)
            .skip(1)
            .collect::<Vec<_>>();

        let arena = Arena::new();
        let root_id = arena.new_dir("/", None);
        let mut cursor = Cursor::with_arena(root_id, arena);

        for line in lines {
            if line.starts_with('$') {
                let mut args = line.split(' ').skip(1);
                match args.next().unwrap() {
                    "cd" => {
                        let dir_name = args.next().unwrap();

                        if dir_name == ".." {
                            cursor.cd_parent();
                        } else {
                            cursor.cd(dir_name);
                        }
                    }
                    "ls" => {
                        //Useless?
                    }
                    _ => {}
                }
            } else {
                let (left, right) = line.split_once(' ').unwrap();

                if left == "dir" {
                    cursor.new_dir(right);
                } else {
                    cursor.new_file(right, left.parse::<u64>().unwrap());
                }
            }
        }

        cursor
    }

    fn first_part(input: Self::InputData) -> Self::OutputData {
        let dirs = input.get_dirs_with_space_pred(100_000, &|need, el| el < need);

        dirs.into_iter()
            .fold(0, |acc, el| acc + input.get_space(el))
    }

    fn second_part(input: Self::InputData) -> Option<Self::OutputData> {
        const TOTAL_SPACE: u64 = 70_000_000;
        const NEED_SPACE: u64 = 30_000_000;

        let mut input = input;
        input.cd_root();

        let used_space = input.get_current_cursor_space();
        let available_space = TOTAL_SPACE - used_space;
        let need_to_free = NEED_SPACE - available_space;

        let space_to_free = input
            .get_dirs_with_space_pred(need_to_free, &|need, el| el > need)
            .into_iter()
            .map(|el| input.get_space(el))
            .min()
            .unwrap();

        Some(space_to_free)
    }
}

pub mod tree {
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::ops::Add;

    pub type NodeId = usize;

    #[allow(dead_code)]
    #[derive(Clone, Debug)]
    enum Node<T> {
        Dir {
            name: String,
            parent: Option<NodeId>,
            children: Vec<NodeId>,
            metadata: RefCell<Option<T>>,
        },
        File {
            name: String,
            data: T,
        },
    }

    impl<T: Clone> Node<T> {
        fn add_child(&mut self, child_id: NodeId) {
            let Node::Dir { children, metadata, .. } = self else {
                panic!("Trying add child to file")
            };

            metadata.take();
            children.push(child_id);
        }

        #[allow(dead_code)]
        fn get_metadata(&self) -> Option<T> {
            let Node::Dir { metadata, .. } = self else {
                panic!("Trying add child to file")
            };

            metadata.borrow().clone()
        }

        fn get_children(&self) -> &[NodeId] {
            let Node::Dir { children, .. } = self else {
                panic!("Trying get children from file")
            };

            children.as_slice()
        }
    }

    #[derive(Clone, Debug)]
    pub struct Arena<T> {
        nodes: RefCell<Vec<Node<T>>>,
    }

    impl<T> Arena<T> {
        pub fn new() -> Self {
            Self {
                nodes: RefCell::new(vec![]),
            }
        }

        pub fn new_dir(&self, name: &str, parent: Option<NodeId>) -> NodeId {
            let next_id = self.nodes.borrow().len();
            self.nodes.borrow_mut().push(Node::Dir {
                name: name.to_string(),
                parent,
                children: vec![],
                metadata: RefCell::new(None),
            });

            next_id
        }

        fn new_file(&self, name: &str, data: T) -> NodeId {
            let next_id = self.nodes.borrow().len();
            self.nodes.borrow_mut().push(Node::File {
                name: name.to_string(),
                data,
            });

            next_id
        }

        fn is_directory(&self, node_id: NodeId) -> bool {
            match self.nodes.borrow()[node_id] {
                Node::Dir { .. } => true,
                Node::File { .. } => false,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Cursor<'arena, T: Clone> {
        root_id: NodeId,
        dir: NodeId,
        arena: Cow<'arena, Arena<T>>,
    }

    #[allow(dead_code)]
    impl<'arena, T: Clone> Cursor<'arena, T> {
        pub fn new(point: NodeId, arena: &'arena Arena<T>) -> Self {
            Self {
                root_id: point,
                dir: point,
                arena: Cow::Borrowed(arena),
            }
        }

        pub fn with_arena(point: NodeId, arena: Arena<T>) -> Self {
            Self {
                root_id: point,
                dir: point,
                arena: Cow::Owned(arena),
            }
        }

        pub fn cd_root(&mut self) {
            self.dir = self.root_id;
        }

        pub fn cd_parent(&mut self) {
            let Node::Dir { parent, .. } = &self.arena.nodes.borrow()[self.dir] else {
                panic!("Cursor point on file")
            };

            let Some(parent) = parent else {
                panic!("Cursor point on root")
            };

            self.dir = *parent;
        }

        pub fn cd(&mut self, dir_name: &str) {
            let Node::Dir { children, .. } = &self.arena.nodes.borrow()[self.dir] else {
                panic!("Cursor point on file")
            };

            let Some(child_id) = children.iter().find(|&&id| {
                if let Node::Dir { name, .. } = &self.arena.nodes.borrow()[id] {
                    dir_name == name
                } else {
                    false
                }
            }) else {
                panic!("No childs with name {}", dir_name)
            };

            self.dir = *child_id;
        }

        pub fn new_dir(&self, name: &str) {
            let dir_id = self.arena.new_dir(name, Some(self.dir));
            self.arena.nodes.borrow_mut()[self.dir].add_child(dir_id);
        }

        pub fn new_file(&self, name: &str, data: T) {
            let file_id = self.arena.new_file(name, data);
            self.arena.nodes.borrow_mut()[self.dir].add_child(file_id);
        }
    }

    impl<'arena, T> Cursor<'arena, T>
    where
        T: Add<Output = T> + Copy + Default + Ord + PartialOrd,
    {
        pub fn get_space(&self, node_id: NodeId) -> T {
            let node = &self.arena.nodes.borrow()[node_id];

            match node {
                Node::Dir {
                    children, metadata, ..
                } => {
                    let mut metadata = metadata.borrow_mut();
                    if let Some(data) = *metadata {
                        return data;
                    }

                    let new_metadata = children
                        .iter()
                        .fold(T::default(), |acc, el| acc + self.get_space(*el));

                    *metadata = Some(new_metadata);

                    new_metadata
                }
                Node::File { data, .. } => *data,
            }
        }

        pub fn get_current_cursor_space(&self) -> T {
            self.get_space(self.dir)
        }

        fn get_child_with_space_pred_child(
            &self,
            node_id: NodeId,
            space: T,
            func: &impl Fn(T, T) -> bool,
            dirs: &mut Vec<NodeId>,
        ) {
            let node_space = self.get_space(node_id);
            let node = &self.arena.nodes.borrow()[node_id];

            if func(space, node_space) {
                dirs.push(node_id);
            }

            for &child in node.get_children() {
                if self.arena.is_directory(child) {
                    self.get_child_with_space_pred_child(child, space, func, dirs);
                }
            }
        }

        pub fn get_dirs_with_space_pred(
            &self,
            space: T,
            func: &impl Fn(T, T) -> bool,
        ) -> Vec<NodeId> {
            let mut dirs = vec![];

            let root_space = self.get_space(self.root_id);
            let root_node = &self.arena.nodes.borrow()[self.root_id];

            if func(space, root_space) {
                dirs.push(self.root_id);
            }

            for &child in root_node.get_children() {
                if self.arena.is_directory(child) {
                    self.get_child_with_space_pred_child(child, space, func, &mut dirs);
                }
            }

            dirs
        }
    }
}
