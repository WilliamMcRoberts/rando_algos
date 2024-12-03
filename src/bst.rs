// Assume these will be present given the problem statement:
// "p and q will exist in the BST."
// let p_val = p.unwrap().borrow().val;
// let q_val = q.unwrap().borrow().val;

// BST properties:
// - LHS < ROOT
// - RHS > ROOT
// We can recursively dive into each side depending the
// value of the current node we are looking at.
// fn dfs(
//     node: &BSTNode,
//     p: i32,
//     q: i32,
//     res_ptr: &mut BSTNode,
// ) {
//     match node {
//         Some(n_rc) => {
//             let n = n_rc.borrow();
//             // update our pointer each iteration
//             *res_ptr = Some(Rc::new(RefCell::new(TreeNode::new(n.val))));

//             // if the root is larger than our params,
//             // then we know we can go left
//             if n.val > p && n.val > q {
//                 dfs(&n.left, p, q, res_ptr)
//             }
//             // if the root is less than our params,
//             // then we know we can go right
//             if n.val < p && n.val < q {
//                 dfs(&n.right, p, q, res_ptr)
//             }
//         }
//         None => (),
//     }
// }

// let mut res = None;
// let mut res_ptr = &mut res;
// dfs(&root, p_val, q_val, &mut res_ptr);
// res

// let p_val = p.unwrap().borrow().val;
// let q_val = q.unwrap().borrow().val;
// let mut root = root;
// let mut res = None;
// while let Some(node) = root {
//     let mut n = node.borrow_mut();
//     res = Some(Rc::new(RefCell::new(TreeNode::new(n.val))));
//     if n.val > p_val && n.val > q_val {
//         root = n.left.take();
//         continue;
//     }
//     if n.val < p_val && n.val < q_val {
//         root = n.right.take();
//         continue;
//     }
//     break;
// }
// res
