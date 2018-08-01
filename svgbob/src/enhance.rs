use focus_char::FocusChar;
use fragments::Fragment;
use location::Location;
use location::Direction::{Bottom, BottomLeft, BottomRight, Left, Right, Top, TopLeft, TopRight};
use block::Block::{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y};
use point_block::PointBlock;
use fragments::{self, line, arc, open_circle, arrow_line, dashed_line};

pub trait Enhance {
    fn enhance(&self) -> (Vec<Fragment>, Vec<Location>);
}

impl<'g> Enhance for FocusChar<'g> {
    fn enhance(&self) -> (Vec<Fragment>, Vec<Location>) {
        let mut elm = vec![];
        let mut consumed = vec![];
        let a = &PointBlock::block(A);
        let _b = &PointBlock::block(B);
        let c = &PointBlock::block(C);
        let _d = &PointBlock::block(D);
        let e = &PointBlock::block(E);
        let _f = &PointBlock::block(F);
        let _g = &PointBlock::block(G);
        let _h = &PointBlock::block(H);
        let _i = &PointBlock::block(I);
        let _j = &PointBlock::block(J);
        let k = &PointBlock::block(K);
        let l = &PointBlock::block(L);
        let m = &PointBlock::block(M);
        let n = &PointBlock::block(N);
        let o = &PointBlock::block(O);
        let _p = &PointBlock::block(P);
        let _q = &PointBlock::block(Q);
        let _r = &PointBlock::block(R);
        let _s = &PointBlock::block(S);
        let _t = &PointBlock::block(T);
        let u = &PointBlock::block(U);
        let _v = &PointBlock::block(V);
        let w = &PointBlock::block(W);
        let _x = &PointBlock::block(X);
        let y = &PointBlock::block(Y);

        let this = || Location::this();
        let top = || Location::go(Top);
        let bottom = || Location::go(Bottom);
        let left = || Location::go(Left);
        let right = || Location::go(Right);
        let top_left = || Location::go(TopLeft);
        let top_right = || Location::go(TopRight);
        let bottom_left = || Location::go(BottomLeft);
        let bottom_right = || Location::go(BottomRight);
        let left2 = || Location::jump(Left,2);
        let right2 = || Location::jump(Right,2);

        // _ underscore
        if self.is('_') {
            //   _|
            if self.right().any("|[") {
                elm.push(line(u, &right().w()));
            }
            //    |_
            if self.left().any("|]") {
                elm.push(line(y, &left().w()));
            }
        }
        if self.any("`'") {
            // for circuitries
            //  +     +    \
            //   `>    '>   `>
            if self.top_left().any("+\\") && self.right().is('>') {
                elm.push(arrow_line(&top_left().m(), &right().f()));
                consumed.push(right());
                if self.top_left().is('\\') {
                    consumed.push(top_left());
                }
            }
            // for circuitries
            //     +    /
            //   <'   <'
            if self.top_right().any("+/") && self.left().is('<') {
                elm.push(arrow_line(&top_right().m(), &left().j()));
                consumed.push(left());
                if self.top_right().is('/') {
                    consumed.push(top_right());
                }
            }
            // For diamond rectangle
            //     .
            //    '
            if self.top_right().any(".,") {
                elm.push(dashed_line(c, &top_right().m()));
                consumed.push(top_right());
                consumed.push(this());
            }
            //   .
            //    '
            if self.top_left().any(".,") {
                elm.push(dashed_line(c, &top_left().m()));
                consumed.push(top_left());
                consumed.push(this());
            }
            //   .'
            if self.left().any(".,") {
                elm.push(dashed_line(c, &left().m()));
                consumed.push(left());
                consumed.push(this());
            }
            //   '.
            if self.right().any(".,") {
                elm.push(dashed_line(c, &right().m()));
                consumed.push(right());
                consumed.push(this());
            }
        }
        if self.any(".,") {
            // for circuitries
            //   <.    <,
            //     +     \
            if self.bottom_right().any("+\\") && self.left().is('<') {
                elm.push(arrow_line(&bottom_right().m(), &left().t()));
                consumed.push(left());
                if self.bottom_right().is('\\') {
                    consumed.push(bottom_right());
                    consumed.push(this());
                }
            }
            // for circuitries
            //   .>    ,>   ,>
            //  +     +    /
            if self.bottom_left().any("+/") && self.right().is('>') {
                elm.push(arrow_line(&bottom_left().m(), &right().p()));
                consumed.push(right());
                if self.bottom_left().is('/') {
                    consumed.push(bottom_left());
                    consumed.push(this());
                }
            }
        }
        // transistor complimentary enhancement
        if self.is('|') {
            //    |    |
            //    <    >
            if self.bottom().any("><") {
                elm.push(line(c, &bottom().m()));
                consumed.push(this());
            }
            //    <    >
            //    |    |
            if self.top().any("><") {
                elm.push(line(w, &top().m()));
                consumed.push(this());
            }
            //    _
            //   |
            if self.top_right().is('_') {
                elm.extend(vec![line(c,w),line(c, e)]);
                consumed.push(this());
            }
            //    _
            //     |
            if self.top_left().is('_') {
                elm.extend(vec![line(c,w),line(a,c)]);
                consumed.push(this());
            }
        } 
        if self.is('/') {
            //      >
            //     /
            if self.top_right().is('>') {
                elm.push(line(u, &top_right().m()));
                consumed.push(this());
            }
            //    /
            //   <
            if self.bottom_left().is('<') {
                elm.push(line(e, &bottom_left().m()));
                consumed.push(this());
            }
        } 
        if self.is('\\') {
            //      \
            //       >
            if self.bottom_right().is('>') {
                elm.push(line(a, &bottom_right().m()));
                consumed.push(this());
            }
            //    <
            //     \
            if self.top_left().is('<') {
                elm.push(line(y, &top_left().m()));
                consumed.push(this());
            }
        }
        // circuitries jump
        //    |
        //   -(-
        //    |
        //
       if self.is('(') && self.top().can_strongly_connect(&W)
            && self.bottom().can_strongly_connect(&C)
            && self.left().can_strongly_connect(&O)
            && self.right().can_strongly_connect(&K)
        {
            elm.extend(vec![arc(c, w, 5), line(k, o)]);
            consumed.push(this());
        }
        // circuitries jump
        //    |
        //   -)-
        //    |
        //
        if self.is(')') && self.top().can_strongly_connect(&W)
            && self.bottom().can_strongly_connect(&C)
            && self.left().can_strongly_connect(&O)
            && self.right().can_strongly_connect(&K)
        {
            elm.extend(vec![arc(w, c, 5), line(k, o)]);
            consumed.push(this());
        }
        //  circle1
        //   _
        //  (_)
        //
        if self.is('_') 
            && self.left().is('(') && self.right().is(')')
            && self.top().is('_'){
            elm.push(open_circle(m, 4));
            consumed.extend(vec![this(), left(), right(),top()]);
        }

        // circle2
        //       .-.
        //      ( + )
        //       '-'
        if self.in_left(2).is('(')
            && self.in_right(2).is(')')
            && self.top().is('-')
            && self.bottom().is('-')
            && self.bottom_left().any("`'")
            && self.bottom_right().is('\'')
            && self.top_left().any(".,")
            && self.top_right().is('.'){
                println!("circle2 matched");

            elm.push(open_circle(m,6));
            consumed.extend(vec![left2(), right2(), top(), bottom(),
                bottom_left(), bottom_right(), top_left(), top_right()]);
        }
        //      .--.
        //     ( +  )
        //      `--'
        //
        //        _
        //      .' '.
        //     (  3  )
        //      `._.'
        //        ___
        //      ,'   `.
        //     /       \
        //    |         |
        //     \       /
        //      `.___,'
        //
        //        ______
        //      ,'      `.
        //     /          \
        //    |            |
        //    |            |
        //     \          /
        //      `.______,'

        (elm, consumed)
    }
}
