import hashlib
import random
import json

def hash_data(data):
    """Erzeugt einen SHA-256 Hash aus Daten."""
    return hashlib.sha256(json.dumps(data, sort_keys=True).encode()).hexdigest()

def generate_node(level_name, level_count, parent_hash=""):
    """Erstellt einen Knoten mit 1-3 Kindern rekursiv."""
    node = {
        "name": f"{level_name}_{random.randint(1,1000)}",
        "hash": "",
        "children": []
    }
    # zufällige Anzahl von Kindern zwischen 1 und 3
    num_children = random.randint(1,3) if level_count > 0 else 0
    
    for _ in range(num_children):
        if level_count == 3:
            # Ebene Länder -> Bundesländer
            child = generate_node("Bundesland", level_count-1, parent_hash)
        elif level_count == 2:
            # Ebene Bundesländer -> Bezirke
            child = generate_node("Bezirk", level_count-1, parent_hash)
        elif level_count == 1:
            # Ebene Bezirke -> Dörfer
            child = generate_node("Dorf", level_count-1, parent_hash)
        else:
            child = None
        if child:
            node["children"].append(child)
    
    # Hash des Knotens = Hash des Namens + Hash der Kinder + Hash des Elternknotens
    node_data = {
        "name": node["name"],
        "children_hashes": [c["hash"] for c in node["children"]],
        "parent_hash": parent_hash
    }
    node["hash"] = hash_data(node_data)
    
    return node

# Erzeuge Baum mit Wurzel = Länder-Ebene (3 Ebenen tiefer)
tree = generate_node("Land", 3)
print(json.dumps(tree, indent=2))
