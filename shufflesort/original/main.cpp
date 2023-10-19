/*
Original C++ solution by @Lapis0875
*/

#include <iostream>
#include <fstream>          // ifstream으로 파일 읽기 위함
#include <cstdio>           // puts() 함수 사용하기 위함
#include <cstring>
#include <vector>           // vector<T> 사용하기 위함
#include <queue>            // queue<T> 사용하기 위함, radix sort 때문.

#define LINES 1000000
#define ALPHABETS 26

using namespace std;

char rule[27] = "mporqtsvuxwzybadcfehgjilkn";
char lookup[26] = {};

/**
 * @brief 두 변수룰 바꾼다.
 * 
 * @param a 바꿀 변수 1
 * @param b 바꿀 변수 2
 */
inline void swap(string& a, string& b)
{
    string temp = a;
    a = b;
    b = temp;
}

/**
 * @brief 두 문자열을 주어진 규칙에 따라 비교한다.
 * 
 * @param a 비교할 문자열. 비교 결과는 이 문자열을 기준으로 한다.
 * @param b 비교할 문자열.
 * @return int @a a가 더 먼저 나오면 -1, 더 나중에 나오면 1을 반환. 같으면 0.
 */
int compare(const string& a, const string& b)
{
    int i = 0;
    int a_length = a.length();
    int b_length = b.length();
    int cmp_length = a_length < b_length ? a_length : b_length;

    char a_index, b_index;
    while (i < cmp_length)
    {
        a_index = lookup[a[i] - 'a'];
        b_index = lookup[b[i] - 'a'];
        if (a_index < b_index)
            return -1;
        else if (a_index > b_index)
            return 1;
        i++;
    }
    if (a_length < b_length)
        return -1;
    else if (a_length > b_length)
        return 1;
    return 0;
}

/**
 * @brief 두 문자열을 주어진 규칙에 따라 비교한다.
 * 
 * @param a 비교할 문자열. 비교 결과는 이 문자열을 기준으로 한다.
 * @param b 비교할 문자열.
 * @return int @a a가 더 먼저 나오면 -1, 더 나중에 나오면 1을 반환. 같으면 0.
 */
int compare(const char* a, const char* b)
{
    int i = 0;
    int a_length = strlen(a);
    int b_length = strlen(b);
    int cmp_length = a_length < b_length ? a_length : b_length;

    char a_index, b_index;
    while (i < cmp_length)
    {
        a_index = lookup[a[i] - 'a'];
        b_index = lookup[b[i] - 'a'];
        if (a_index < b_index)
            return -1;
        else if (a_index > b_index)
            return 1;
        i++;
    }
    if (a_length < b_length)
        return -1;
    else if (a_length > b_length)
        return 1;
    return 0;
}

/**
 * @brief 퀵 정렬의 파티션 함수
 * 
 * @param lines 데이터가 들어있는 벡터
 * @param start 이 파티션의 시작 인덱스
 * @param end 이 파티션의 끝 인덱스
 * @return int 다음 피벗의 인덱스
 */
inline int partition(vector<string>& lines, int start, int end)
{
    int low = start + 1;
    int high = end;
    string pivot = lines[start];

    while (low < high)
    {
        for(; low <= end && compare(lines[low], pivot) < 0; low++);
        for(; high >= start && compare(lines[high], pivot) > 0; high--);

        // cout << "partition << low(" << low << "), high(" << high << ") ";

        if (low < high)
        {
            swap(lines[low], lines[high]);
            // cout << "swap! low <-> high";
        }
    }
    swap(lines[start], lines[high]);
    // cout << "swap! start <-> high";
    // cout << endl;
    return high;
}

/**
 * @brief 퀵 정렬의 파티션 함수
 * 
 * @param lines 데이터가 들어있는 벡터
 * @param start 이 파티션의 시작 인덱스
 * @param end 이 파티션의 끝 인덱스
 * @return int 다음 피벗의 인덱스
 */
inline int partition(const char* (&lines)[LINES], int start, int end)
{
    int low = start + 1;
    int high = end;
    const char* pivot = lines[start];

    while (low < high)
    {
        for(; low <= end && compare(lines[low], pivot) < 0; low++);
        for(; high >= start && compare(lines[high], pivot) > 0; high--);

        // cout << "partition << low(" << low << "), high(" << high << ") ";

        if (low < high)
        {
            swap(lines[low], lines[high]);
            // cout << "swap! low <-> high";
        }
    }
    swap(lines[start], lines[high]);
    // cout << "swap! start <-> high";
    // cout << endl;
    return high;
}

/**
 * @brief 퀵 정렬 알고리즘
 * 
 * @param lines 문자열 데이터의 벡터
 * @param start 정렬 시작 인덱스
 * @param end 정렬 끝 인덱스
 */
inline void quicksort(vector<string>& lines, int start, int end)
{
    if (start >= end)
        return;         // I don't want additional indent.
    
    int pivot = partition(lines, start, end);
    quicksort(lines, start, pivot - 1);
    quicksort(lines, pivot + 1, end);
}

/**
 * @brief 퀵 정렬 알고리즘
 * 
 * @param lines 문자열 데이터의 벡터
 * @param start 정렬 시작 인덱스
 * @param end 정렬 끝 인덱스
 */
inline void quicksort(const char* (&lines)[LINES], int start, int end)
{
    if (start >= end)
        return;         // I don't want additional indent.
    
    int pivot = partition(lines, start, end);
    quicksort(lines, start, pivot - 1);
    quicksort(lines, pivot + 1, end);
}

/**
 * @brief 부분 정렬된 배열 @a arr을 병합한다.
 * 
 * @param arr 병합될 배열
 * @param left 왼쪽 인덱스
 * @param mid 중간 인덱스
 * @param right 오른쪽 인덱스
 */
inline void merge(vector<string>& arr, int left, int mid, int right)
{
    vector<string> sorted(right - left + 1);
    int i = left;
    int j = mid + 1;
    int k = 0;

    while (i <= mid && j <= right)
        sorted[k++] = compare(arr[i], arr[j]) <= 0 ? arr[i++] : arr[j++];
    
    while (i <= mid)
        sorted[k++] = arr[i++];
    
    while (j <= right)
        sorted[k++] = arr[j++];
    
    for (int idx = left, k = 0; idx <= right; ++idx, ++k)
        arr[idx] = sorted[k];
}

/**
 * @brief 병합 정렬 알고리즘
 * 
 * @param arr 정렬할 배열
 * @param left 정렬할 시작 인덱스
 * @param right 정렬할 끝 인덱스
 */
inline void mergesort(vector<string>& arr, int left, int right)
{
    if (left >= right)
        return;
    int mid = (left + right) / 2;

    mergesort(arr, left, mid);
    mergesort(arr, mid + 1, right);
    merge(arr, left, mid, right);
}

/**
 * @todo 기수 정렬 알고리즘 구현
 * @brief 기수 정렬 알고리즘.
 * 
 * @param arr 정렬할 문자열 배열 (벡터).
 */
inline void radixsort(vector<string>& arr)
{
    queue<string> buckets[26];
    int n = arr.size();

    string e;
    for (int d = 0; d < 20; d++)
    {
        for (int i = 0; i < n; i++)
        {
            e = arr[i];
            buckets[lookup[e[d] - 'a']].push(e);
        }
        for (int b = 0, i = 0; b < 26; b++)
        {
            while (!buckets[b].empty())
            {
                arr[i++] = buckets[b].front();
                buckets[b].pop();
            }
        }
    }
}

/**
 * @brief 프로그램의 진입점.
 * 
 * @param argc 프로그램 인자 개수
 * @param argv 프로그램 인자 배열
 * @return int 종료 코드
 */
int main(int argc, char* argv[])
{
    if (argc < 1)               // 파일명이 주어지지 않았을 시 종료한다.
    {
        cout << "Usage : " << argv[0] << " <filename>" << endl;
        return 1;
    }

    const char* default_dictionary = "abcdefghijklmnopqrstuvwxyz";
    char dict[ALPHABETS] = {};
    // 단일 루프로 해결할 수 있을까?
    for (int i = 0; i < ALPHABETS; i += 2)
    {
        dict[i] = default_dictionary[i + 1];
        dict[i+1] = default_dictionary[i];
    }
    for (int i = 0; i < 13; i++)
        rule[i] = dict[13 + i];
    
    // cout << "Building lookup tagble..." << endl;
    for (int i = 0; i < 13; i++)
        rule[13 + i] = dict[i];
    
    // lookup table 만들기
    for (int i = 0; i < ALPHABETS; i++)
        lookup[rule[i] - 'a'] = i;
    
    // 파일 읽기
    // cout << "Start reading file : " << argv[1] << endl;
    ifstream text_file(argv[1]);
    string s;
    vector<string> lines;
    if (text_file.is_open())
    {
        // cout << "File is open!" << endl;
        int i = 0;
        while (getline(text_file, s))
        {
            // cout << "line[" << i << "] : " << line << endl;
            // lines[i] = s.data();
            lines.push_back(s);
            i++;
        }
        text_file.close();
    }

    // cout << "Start sorting <QuickSort>" << endl;
    quicksort(lines, 0, 999999);
    s.clear();
    for (int i = 0; i < LINES; i++)
        s.append(lines[i]).append("\n");
    
    puts(s.data());
    return 0;
}
