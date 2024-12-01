import scala.io.Source
import scala.collection.mutable.{Map => MMap}

object day5 {
  def main(args: Array[String]): Unit = {
    val data = Source.fromFile("src/main/resources/day5.txt").mkString
    // var dataList: List[(String,String)] = data.split("\n").map(x => (x.split(" -> ")(0).split(",").map(_.toInt), x.split(" -> ")(1).split(",").map(_.toInt))).toList
    var dataList = data.split("\n").map(_.split(" -> ").toList
      .map(x => x.split(",").map(_.toInt).toList))
      .toList.map(x => (x(0)(0), x(0)(1), x(1)(0), x(1)(1)))//.flatMap(
      // .map(x => (x(0).asDigit,x(1).asDigit,x(2).asDigit,x(3).asDigit))

    // println(dataList)

    val maxDim = 990

    def makeGridMap(Dx: Int, Dy: Int): collection.mutable.Map[(Int, Int), Int]  = {
      var xs = (0 to Dx).toList
      var ys = (0 to Dy).toList
      var m = xs.flatMap(x => ys.map(y => ((x, y),0))).toMap
      var out = MMap(m.toSeq: _*)
      out
    }

    var grid = makeGridMap(maxDim, maxDim)

    def go(l: List[(Int, Int, Int, Int)], grid: MMap[(Int,Int), Int]): Int = l match {
      case Nil => {(grid.values.map(x => if (x >= 2) 1 else 0 ).sum)}
      case (x1: Int, y1: Int, x2: Int, y2: Int) :: t if x1 == x2 & y1 < y2 => {
        var ngrid = grid
        for (y <- (y1 to y2)) {
          ngrid = ngrid + ((x1, y) -> (ngrid((x1, y)) + 1))          
        }
        go(t, ngrid)
      }
      case (x1: Int, y1: Int, x2: Int, y2: Int) :: t if x1 == x2 & y1 > y2 => {
        var ngrid = grid
        for (y <- (y2 to y1)) {
          ngrid = ngrid + ((x1, y) -> (ngrid((x1, y)) + 1))          
        }
        go(t, ngrid)
      }
      case (x1: Int, y1: Int, x2: Int, y2: Int) :: t if y1 == y2 & x1 < x2 => {
        var ngrid = grid
        for (x <- (x1 to x2)) {
          ngrid = ngrid + ((x, y1) -> (ngrid((x, y1)) + 1))          
        }
        go(t, ngrid)
      }
      case (x1: Int, y1: Int, x2: Int, y2: Int) :: t if y1 == y2 & x1 > x2 => {
        var ngrid = grid
        for (x <- (x2 to x1)) {
          ngrid = ngrid + ((x, y1) -> (ngrid((x, y1)) + 1))          
        }
        go(t, ngrid)
      }
      case (x1, y1, x2, y2) :: t => go(t, grid)
    }
    println(go(dataList, grid))

  }
}

object day5B {
  def main(args: Array[String]): Unit = {
    val data = Source.fromFile("src/main/resources/day5.txt").mkString
    // var dataList: List[(String,String)] = data.split("\n").map(x => (x.split(" -> ")(0).split(",").map(_.toInt), x.split(" -> ")(1).split(",").map(_.toInt))).toList
    var dataList = data.split("\n").map(_.split(" -> ").toList
      .map(x => x.split(",").map(_.toInt).toList))
      .toList.map(x => (x(0)(0), x(0)(1), x(1)(0), x(1)(1)))//.flatMap(
      // .map(x => (x(0).asDigit,x(1).asDigit,x(2).asDigit,x(3).asDigit))

    // println(dataList)

    val maxDim = 990

        def makeGridMap(Dx: Int, Dy: Int): collection.immutable.Map[(Int, Int), Int]  = {
      var xs = (0 to Dx).toList
      var ys = (0 to Dy).toList
      var m = xs.flatMap(x => ys.map(y => ((x, y),0))).toMap
      // var out = MMap(m.toSeq: _*)
      m
        }

    var grid = makeGridMap(maxDim, maxDim)

    def go(l: List[(Int, Int, Int, Int)], grid: Map[(Int,Int), Int]): Int = l match {
      case Nil => {(grid.values.map(x => if (x >= 2) 1 else 0 ).sum)}
      case (x1: Int, y1: Int, x2: Int, y2: Int) :: t if x1 == x2 & y1 < y2 => {
        var ngrid = grid
        for (y <- (y1 to y2)) {
          ngrid = ngrid + ((x1, y) -> (ngrid((x1, y)) + 1))          
        }
        go(t, ngrid)
      }
      case (x1: Int, y1: Int, x2: Int, y2: Int) :: t if x1 == x2 & y1 > y2 => {
        var ngrid = grid
        for (y <- (y2 to y1)) {
          ngrid = ngrid + ((x1, y) -> (ngrid((x1, y)) + 1))          
        }
        go(t, ngrid)

      }
      case (x1: Int, y1: Int, x2: Int, y2: Int) :: t if y1 == y2 & x1 < x2 => {
        var ngrid = grid
        for (x <- (x1 to x2)) {
          ngrid = ngrid + ((x, y1) -> (ngrid((x, y1)) + 1))          
        }
        go(t, ngrid)
      }
      case (x1: Int, y1: Int, x2: Int, y2: Int) :: t if y1 == y2 & x1 > x2 => {
        var ngrid = grid
        for (x <- (x2 to x1)) {
          ngrid = ngrid + ((x, y1) -> (ngrid((x, y1)) + 1))          
        }
        go(t, ngrid)
      }
      case (x1: Int, y1: Int, x2: Int, y2: Int) :: t if (y1 - y2).abs ==  (x1 - x2).abs => {
        var ngrid = grid
        if (x1 < x2 & y1 < y2) {
          for ((x,y) <- ((x1 to x2), (y1 to y2)).zipped) {
            // print("A", x1, x2, y1, y2, x,y)
            ngrid = ngrid + ((x, y) -> (ngrid((x, y)) + 1))
          }
        } else if (x2 < x1 & y1 < y2) {
          for ((x,y) <- ((x1 to x2 by -1), (y1 to y2)).zipped) {
            // print("B", x1, x2, y1, y2, x,y)
            ngrid = ngrid + ((x, y) -> (ngrid((x, y)) + 1))
          }
        } else if (x1 < x2 & y2 < y1) {
          for ((x,y) <- ((x1 to x2), (y1 to y2 by -1)).zipped) {
            // print("C", x1, x2, y1, y2, x,y)
            ngrid = ngrid + ((x, y) -> (ngrid((x, y)) + 1))
          }
        } else if (x2 < x1 & y2 < y1) {
          for ((x,y) <- ((x1 to x2 by -1), (y1 to y2 by -1)).zipped) {
            // print("D", x1, x2, y1, y2, x,y)
            ngrid = ngrid + ((x, y) -> (ngrid((x, y)) + 1))
          }
        }
        go(t, ngrid)


      }
      // case (x1: Int, y1: Int, x2: Int, y2: Int) :: t if abs(y1 - y2) == abs(x1 - x2) & x1 > x2 & y1 > y2=> {
      //   go(t, grid.map((k,v) => {
      //     var (x,y) = k
      //     var ((x1 to x2), (y1 to y2).toList)
      //     if ((x1 - x) ==  & y == y1) ((x,y),v+1) else ((x,y),v)
      //   }))
      // }
      case (x1, y1, x2, y2) :: t => go(t, grid)
    }
    println(go(dataList, grid))

  }
}
