// #include <bits/stdc++.h>
#include <iostream>
#include <vector>

using namespace std;
int main()
{

    int as = 0;

    // Wrong code
    int abcd = 0;
    int abCd = 0;
    int ab_cd = 0;
    int abCD = 0;
    const int ABcD = 0;

    // Correct code
    int abcd = 0;
    int abCd = 0;
    // int ab_cd = 0;
    int abCd = 0;
    const int ABCD = 0;

    int count = 10;

    for (size_t i = 0; i < count; i++)
    {

        cout << "Hello World" << endl;
    }

    vector<int> range(count); // Create a vector with 10 elements (0 to 9)
    for (auto x : range)
    {

        cout << "Hello World" << endl;
    }

    for (auto x : range)
    {
        cout << "Hello World" << endl;
    }

    main2();
}

int main2()
{
    return 1;
}