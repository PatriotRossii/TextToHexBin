text = [ord(e) for e in input("Enter text: ")]

print("Your text in bin:", " ".join([f"{e:b}" for e in text]))
print("Your text in hex:", " ".join([f"{e:x}" for e in text]))