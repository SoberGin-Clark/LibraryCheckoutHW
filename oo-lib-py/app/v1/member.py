from .catalog import Catalog

class Member:
    def __init__(self, name): # Initializes the member class
        self.name = name
        self.myBorrows = []
    
    def borrow(self, id: str): # Takes an item out for borrowing
        if len(self.myBorrows) >= 5:
            raise ValueError
        
        dup = False
        for thing in self.myBorrows:
            if id == thing:
                dup = True

        if dup == False:
            self.myBorrows.append(id)
        else: # If no duplicates, add. If not, raise an error
            raise ValueError
    
    def borrowed_ids(self) -> str: # Returns a list of all the ids borrowed currently
        return self.myBorrows
    
    def list_details(self, cat: Catalog): # Provides a list of the details of each item borrowed using a given catalog
        endList = []

        for thing in self.myBorrows:
            title = cat.get(thing).title
            days = " (" + str(cat.get(thing).days_allowed()) + " days)"
            
            finalStr = title + days
            endList.append(finalStr)
        
        return endList
    
    def return_item(self, id: str): # Returns a borrowed item
        match = False
        for thing in self.myBorrows:
            if id in thing:
                match = True
        
        if match == True:
            self.myBorrows.remove(id)
        else: # Fails if the item isn't in the list of borrowed items
            raise ValueError
