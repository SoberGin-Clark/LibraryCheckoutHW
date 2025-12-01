
from .items import Item

class Catalog: # Catalog class
    
    def __init__(self): # Constructor
        self.catList = []

    def add(self, newItem) -> None: # Add a new item to the catalog. Fails if item with given ID already exists
        dup = False

        for i in self.catList:
            if newItem.id.value == i.id.value:
                dup = True
        
        if dup == False:
            self.catList.append(newItem)
        else:
            raise ValueError
    
    def get(self, id: str) -> Item: # Provides the information about an item. Fails if no such item with the given id exists
        match = False
        target = 0
        for index, item in enumerate(self.catList):
            if item.id.value == id:
                match = True
                target = index
    
        if match == False:
            raise ValueError("No such item with id ", id, " exists!")
        else:
            return self.catList[target]
        
