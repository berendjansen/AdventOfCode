import scala.io.Source

object day2 {
  def main(args: Array[String]): Unit = {
    val data = Source.fromFile("src/main/resources/day2.txt").mkString
    var dataList = data.split("\n").toList

    def parseInstruction(ins: String): (String, Int) = {
      (ins.split(" ")(0), ins.split(" ")(1).toInt)
    }

    def applyInstruction(ins: (String, Int), posX: Int, posY: Int): (Int, Int) = ins match {
      case (_, 0) => (posX, posY)
      case (dir, step) if dir == "forward" => (posX + step, posY)
      case (dir, step) if dir == "down" => (posX, posY + step)
      case (dir, step) if dir == "up" => (posX, posY - step)
    }

    def go(l: List[String], posX: Int, posY: Int): (Int, Int) = l match {
      case Nil => (posX, posY)
      case h :: t => {
        var (newX, newY) = applyInstruction(parseInstruction(h), posX, posY)
        go(t, newX, newY)
      }
    }
    var out = go(dataList, 0, 0)
    println(out._1 * out._2)
  }
}

object day2B {
  def main(args: Array[String]): Unit = {
    val data = Source.fromFile("src/main/resources/day2.txt").mkString
    var dataList = data.split("\n").toList

    def parseInstruction(ins: String): (String, Int) = {
      (ins.split(" ")(0), ins.split(" ")(1).toInt)
    }

    def applyInstruction(ins: (String, Int), posX: Int, posY: Int, aim: Int): (Int, Int, Int) = ins match {
      case (_, 0) => (posX, posY, aim)
      case (dir, step) if dir == "forward" => (posX + step, posY + aim * step, aim)
      case (dir, step) if dir == "down" => (posX, posY, aim + step)
      case (dir, step) if dir == "up" => (posX, posY, aim - step)
    }

    def go(l: List[String], posX: Int, posY: Int, aim: Int): (Int, Int, Int) = l match {
      case Nil => (posX, posY, aim)
      case h :: t => {
        var (newX, newY, newAim) = applyInstruction(parseInstruction(h), posX, posY, aim)
        go(t, newX, newY, newAim)
      }
    }
    var out = go(dataList, 0, 0, 0)
    println(out._1 * out._2)
  }
}
