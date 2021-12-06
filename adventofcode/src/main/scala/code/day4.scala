import scala.io.Source
import scala.collection.mutable.ListBuffer
import breeze.linalg.{DenseMatrix, sum, Axis, any}

object day4 {
  def main(args: Array[String]): Unit = {
    val data = Source.fromFile("src/main/resources/day4.txt").mkString
    var dataList: List[String] = data.split("\n\n").toList

    var numbers = dataList.head.split(",").toList.map(_.toInt)
    var cards = dataList.tail.map(_.split("\n")
      .map(_.split(" ").filter(x => x != "").toArray)
      .toArray.flatten.map(_.toInt))

    var matrices = cards.map(x => new DenseMatrix(5,5,x))

    println(f"Example matrix: ${matrices(0)}")
    println(f"Numbers: ${numbers}")    

    def updateCards(inputCards: List[DenseMatrix[Int]], number: Int): List[DenseMatrix[Int]] = {
      inputCards.map(x => {
        x(x:==number) := -1
        x
      })
    }

    def checkCard(card: DenseMatrix[Int]): Boolean = {
      var rows = sum(card, Axis._0)
      var columns = sum(card, Axis._1)
      any(rows:==(-5)) | any(columns:==(-5))
    }

    def checkWinner(cards: List[DenseMatrix[Int]], lastNum: Int): Int = cards match {
      case Nil => -1
      case h :: t => {
        if (checkCard(h)) {
          sum(h(h:!=(-1))) * lastNum
        } else checkWinner(t, lastNum)
      }
    }

    def playGame(numbers: List[Int], inputCards: List[DenseMatrix[Int]]): Int = numbers match {
      case Nil => 0
      case h :: t => {
        var updatedCards = updateCards(inputCards, h)
        val results = checkWinner(updatedCards, h)
        if (results == -1) playGame(t, updatedCards)
        else results
      }
    }
    println(playGame(numbers, matrices))
  }
}

object day4B {
  def main(args: Array[String]): Unit = {
    val data = Source.fromFile("src/main/resources/day4.txt").mkString
    var dataList: List[String] = data.split("\n\n").toList

    var numbers = dataList.head.split(",").toList.map(_.toInt)
    var cards = dataList.tail.map(_.split("\n")
      .map(_.split(" ").filter(x => x != "").toArray)
      .toArray.flatten.map(_.toInt))

    var matrices = cards.map(x => new DenseMatrix(5,5,x))

    println(f"Example matrix: ${matrices(0)}")
    println(f"Numbers: ${numbers}")    

    def updateCards(inputCards: List[DenseMatrix[Int]], number: Int): List[DenseMatrix[Int]] = {
      inputCards.map(x => {
        x(x:==number) := -1
        x
      })
    }

    def checkCard(card: DenseMatrix[Int]): Boolean = {
      var rows = sum(card, Axis._0)
      var columns = sum(card, Axis._1)
      any(rows:==(-5)) | any(columns:==(-5))
    }

    def removeWinners(inputCards: List[DenseMatrix[Int]]): List[DenseMatrix[Int]] = {
      inputCards.filter(!checkCard(_))
    }

    def playGame(numbers: List[Int], inputCards: List[DenseMatrix[Int]], prevLosers: List[DenseMatrix[Int]]): Int = numbers match {
      case Nil => 0
      case h :: t => {
        var updatedCards = updateCards(inputCards, h)
        val losers = removeWinners(updatedCards)
        if (losers.length == 0) (sum(prevLosers(0)(prevLosers(0):!=(-1))) * h)
        else playGame(t, updatedCards, losers)
      }
    }
    println(playGame(numbers, matrices, matrices))
  }
}
