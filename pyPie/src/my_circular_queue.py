class MyCircularQueue:
    def __init__(self, size):
        self.queue = [None] *size
        self.limit = size

    def Front(self):
        return self.queue[0] if len(self.queue) > 0 else -1

    def Rear(self):
        return self.queue[-1] if len(self.queue) > 0 else -1

    def enQueue(self, value):
        if self.isFull():
            return False
        self.queue[-1] =value
        return self.queue[-1] == value

    def deQueue(self):
        if len(self.queue) > 0:
            del self.queue[-1]
            return True
        else:
            return False

    def isFull(self):
        return self.getSize() >= self.limit

    def isEmpty(self):
        return self.getSize() == 0

    def getSize(self):
        return len(self.queue)


obj = MyCircularQueue(3)


def setUpClass():
    pass
    # cls.obj =  MyCircularQueue(3)


def test_empty_queue_front():
    assert obj.Front() == -1


def test_empty_queue_rear():
    assert obj.Rear() == -1


def test_queue_is_empty():
    assert obj.isEmpty() == True


def test_queue_is_not_full():
    assert obj.isFull() == False


def test_enqueue_one():
    assert obj.enQueue(1) == True


def test_queue_is_not_empty():
    assert obj.isFull() == False


def test_enqueue_two():
    assert obj.enQueue(2) == True


def test_enqueue_three():
    assert obj.enQueue(3) == True


def test_queue_is_full():
    assert obj.isFull() == True


def test_enqueue_four():
    assert obj.enQueue(4) == False


def test_queue_values():
    assert obj.queue == [1, 2, 3]


def test_last_queue_value():
    assert obj.Rear() == 3


def test_first_queue_value():
    assert obj.Front() == 1


def test_remove_last_queue_value():
    assert obj.deQueue() == True


def test_add_value_to_queue():
    assert obj.enQueue(4) == True


def test_new_queue_values():
    assert obj.queue == [1, 2, 4]


def test_new_last_queue_value():
    assert obj.Rear() == 4


print(obj.Front())  # return  -1
print(obj.Rear())  # return  -1
print(obj.isEmpty())  # return True
print(obj.isFull())  # return False
print(obj.enQueue(1))  # return True
print(obj.isEmpty())  # return False
print(obj.enQueue(2))  # return True
print(obj.enQueue(3))  # return True
print(obj.isFull())  # return True
print(obj.enQueue(4))  # return False
print(obj.queue)  # return [1,2,3]
print(obj.Rear())  # return 3
print(obj.Front())  # return 1
print(obj.deQueue())  # return True
print(obj.enQueue(4))  # return True
print(obj.queue)  # return [1,2,4]
print(obj.Rear())  # return 4
print(obj.Rear())  # return 4
