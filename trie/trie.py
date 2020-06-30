class Trie:
    def __init__(self, char: str = ''):
        self.char = ''
        self.children = {}
        self.end = False


    def insert(self, string):
        current = self
        for char in string:
            children = current.children
            if not children.get(char): children[char] = Trie(char)
            current = children[char]
        current.end = True
        return self
    

    def search(self, string):
        current = self
        for char in string:
            children = current.children
            if not children.get(char): return False
            current = children[char]
        return current.end


    def starts_with(self, string):
        current = self
        for char in string:
            children = current.children
            if not children.get(char): return None
            current = children[char]
        return current