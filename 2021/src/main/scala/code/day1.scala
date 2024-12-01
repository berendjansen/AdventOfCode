import scala.io.Source

object day1A {
  def main(args: Array[String]): Unit = {
    var data = Source.fromFile("src/main/resources/day1.txt").mkString
    var dataList = data.split("\n").map(x => x.toInt).toList

    @annotation.tailrec
    def go(l: List[Int], prev: Int, count: Int): Int = l match {
      case Nil => count
      case h :: t => if (h > prev) go(t, h, count + 1) else go(t, h, count)
    }
    val output = go(dataList, dataList.head + 1, 0)
    print(output)
  }
}

object day1B {
  def main(args: Array[String]): Unit = {
    var data = Source.fromFile("src/main/resources/day1.txt").mkString
    var dataList = data.split("\n").map(x => x.toInt).toList

    @annotation.tailrec
    def go(l: List[Int], prevSum: Int, count: Int): Int = l match {
      case Nil => count
      case h :: t if t.length < 2 => count
      case h :: t => {
        val currSum = l.take(3).sum
        if (currSum > prevSum) {
          go(t, currSum, count + 1)
        }
        else go(t, currSum, count)
      }
    }
    val output = go(dataList, dataList.take(3).sum + 1, 0)
    print(output)
  }
}
