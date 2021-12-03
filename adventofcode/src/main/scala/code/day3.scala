import scala.io.Source

object day3 {
  def main(args: Array[String]): Unit = {
    val data = Source.fromFile("src/main/resources/day3.txt").mkString
    var dataList: List[String] = data.split("\n").toList
    // println(dataList.map(x => x))
    // var out = go(dataList, 0, 0)
    var out = dataList.map(s => s.toList.map(_.asDigit))
    var summed = out.reduce((x, y) => (x, y).zipped.map(_+_))

    def getGamma(l: List[Int], n: Int): Int = {
      var tmpVal: String = l.map(x => if (x > n/2) 1 else 0).mkString
      Integer.parseInt(tmpVal, 2)
    }

    def getEpsilon(l: List[Int], n:Int): Int = {
      Integer.parseInt(l.map(x => if (x <= n/2) 1 else 0).mkString, 2)
    }

    var output = getGamma(summed, dataList.length) * getEpsilon(summed, dataList.length)

    println(output)

  }
}

object day3B {
  def main(args: Array[String]): Unit = {
    val data = Source.fromFile("src/main/resources/day3example.txt").mkString
    var dataList: List[String] = data.split("\n").toList

    def getCounts(l: List[String]): List[Int] = {
      var tmp = l.map(s => s.toList.map(_.asDigit))
      tmp.reduce((x, y) => (x, y).zipped.map(_+_))
    }

    def getPattern(l: List[Int], n: Int, i: Int, f: (Int, Int) => Boolean): String = l match {
      case Nil => ""
      case _ => List(l(i)).map(x => if (f(x,n)) 1 else 0).mkString
    }

    def getMatches(subPattern: String, values: List[String]): List[String] = {
      values.filter(x => x.take(subPattern.length) == subPattern)
    }

    def getRating(l: List[String], f: (Int, Int) => Boolean): Int = {
      @annotation.tailrec
      var pattern = ""
      def go(l: List[String], i: Int): String = l match {
        case List(_) if l.length == 1 => l.head
        case _ => {
          var counts = getCounts(l.map(_.take(i)))
          pattern = pattern + getPattern(counts, l.length, i-1, f)
          var matches = getMatches(pattern, l)
          go(matches, i+1)
        }
      }
      Integer.parseInt(go(l, 1), 2)
    }

    println(f"CO2 Rating: ${getRating(dataList, (x,n) => x >= n-x)}")
    println(f"Oxygen Rating:  ${getRating(dataList, ((x,n) => !(x >= n-x)))}")
    println(f"Answer : ${getRating(dataList, ((x,n) => x >= n-x)) * getRating(dataList, (x,n) => !(x >= n-x))}")
  }
}
