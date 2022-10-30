CXX = clang++
CXXFLAGS = -std=c++2b \
	-Wall -Weverything \
	-Wno-shadow-field-in-constructor \
	-Wno-c++98-compat
SRC = $(wildcard *.cpp)
OBJ = $(SRC:.cpp=.o)

a.out: $(OBJ)
	$(CXX) $(OBJ)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $<
