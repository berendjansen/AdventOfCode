import scala.io.Source
import scala.collection.mutable.{Map => MMap}

object day6 {
  def main(args: Array[String]): Unit = {
    val data = Source.fromFile("src/main/resources/day6example.txt").mkString
    // var dataList: List[(String,String)] = data.split("\n").map(x => (x.split(" -> ")(0).split(",").map(_.toInt), x.split(" -> ")(1).split(",").map(_.toInt))).toList
    var dataList = data.split(",").toList.map(_.trim).map(_.toInt)

    println(dataList)

    var counts = dataList.groupBy(identity).mapValues(_.size).toMap

    println(counts)
  }
}
