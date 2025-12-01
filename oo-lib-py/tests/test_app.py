
import pytest
from app.v1.items import Id, Book, Dvd
from app.v1.catalog import Catalog
from app.v1.member import Member

def setup_sample():
    cat = Catalog()
    cat.add(Book(Id("B1"), "Rust for Humans"))
    cat.add(Book(Id("B2"), "Pythonic Patterns"))
    cat.add(Dvd(Id("D1"), "Taking Flight"))
    return cat

def test_add_and_get():
    cat = setup_sample()
    assert cat.get("B1").title == "Rust for Humans"
    assert cat.get("D1").days_allowed() == 7

def test_duplicate_id_rejected():
    cat = setup_sample()
    with pytest.raises(ValueError):
        cat.add(Book(Id("B1"), "Duplicate"))

def test_member_borrow_and_return():
    cat = setup_sample()
    m = Member("Clark")
    m.borrow("B1")
    assert m.borrowed_ids() == ["B1"]
    m.return_item("B1")
    assert m.borrowed_ids() == []

def test_member_cannot_borrow_twice():
    m = Member("X")
    m.borrow("K1")
    with pytest.raises(ValueError):
        m.borrow("K1")

def test_member_limit_enforced():
    m = Member("X")
    for i in range(5):
        m.borrow(f"K{i}")
    with pytest.raises(ValueError):
        m.borrow("K5")

def test_details_lines():
    cat = setup_sample()
    m = Member("Y")
    m.borrow("B2")
    lines = m.list_details(cat)
    assert len(lines) == 1
    assert "Pythonic Patterns" in lines[0]
    assert "(14 days)" in lines[0]

# My Gin-erated tests
def test_gin_many_books_test():
    cat = Catalog()
    cat.add(Book(Id("F1"), "Foundation"))
    cat.add(Book(Id("F2"), "Foundation and Empire"))
    cat.add(Book(Id("F3"), "Second Foundation"))
    cat.add(Book(Id("F4"), "Foundation's Edge"))
    cat.add(Book(Id("F5"), "Foundation and Earth"))

    g = Member("Gin")
    g.borrow("F1")
    g.borrow("F2")
    g.borrow("F3")
    g.borrow("F4")

    lines = g.list_details(cat)
    assert len(lines) == 4 
    assert "Foundation" in lines[0]
    assert "Empire" in lines[1]

    g.return_item("F2")
    g.borrow("F5")
    lines = g.list_details(cat)
    assert len(lines) == 4
    assert "Empire" not in lines[1]

def test_gin_no_unborrowed_returns_test():
    cat = Catalog()
    cat.add(Book(Id("F1"), "Foundation"))
    cat.add(Book(Id("F2"), "Foundation and Empire"))
    cat.add(Book(Id("F3"), "Second Foundation"))
    cat.add(Book(Id("F4"), "Foundation's Edge"))
    cat.add(Book(Id("F5"), "Foundation and Earth"))

    g = Member("Gin")
    g.borrow("F1")
    g.borrow("F2")
    g.borrow("F3")
    g.borrow("F4")

    with pytest.raises(ValueError):
        g.return_item("F5")