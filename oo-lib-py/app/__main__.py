
from app.v1.items import Id, Book, Dvd
from app.v1.catalog import Catalog
from app.v1.member import Member

def demo() -> None:
    cat = Catalog()

    # Books allowed 7 days
    cat.add(Book(Id("B1"), "Rust for Humans"))
    cat.add(Book(Id("B2"), "Pythonic Patterns"))

    # Dvds allowed 14 days
    cat.add(Dvd(Id("D1"), "Taking Flight"))
    cat.add(Dvd(Id("D2"), "Patterns in Motion"))

    m = Member("Clark") # Add a member
    m.borrow("B1") # Borrow two items for the new member
    m.borrow("D1")
    print(m.name, "has borrowed:", m.borrowed_ids()) # Print list of items that are borrowed by the member
    for line in m.list_details(cat): # Print out a list of the items and their details
        print(" •", line)
    m.return_item("B1") # Return an item
    print(m.name, "has borrowed:", m.borrowed_ids()) # Check the number of borrowed again to see the previous was returned

if __name__ == "__main__": # Run the main function
    demo()
